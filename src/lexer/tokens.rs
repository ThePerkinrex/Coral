use std::collections::VecDeque;

use logos::{Lexer, Logos, Source};

use crate::{
    fs::FileId,
    span::Span,
    transaction::{Transaction, Transactionable},
};

pub type Data<'source, Token> = (
    Result<Token, <Token as Logos<'source>>::Error>,
    Span,
    &'source <<Token as Logos<'source>>::Source as Source>::Slice,
);
pub type RefData<'r, 'source, Token> = (
    Result<&'r Token, &'r <Token as Logos<'source>>::Error>,
    Span,
    &'source <<Token as Logos<'source>>::Source as Source>::Slice,
);

pub trait TokenReader<'source, Token: Logos<'source>> {
    fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>>;
    fn get_current_span(&self) -> Option<Span>;
    fn current(&self) -> Option<RefData<'_, 'source, Token>>;
    fn current_slice(&self) -> Option<&'source <Token::Source as Source>::Slice>;
    fn extras(&self) -> &Token::Extras;
    fn advance(&mut self);
    fn is_eoi(&self) -> bool;
}

pub struct Tokens<'source, Token: Logos<'source>> {
    file: FileId,
    lexer: Lexer<'source, Token>,
    current: Option<Data<'source, Token>>,
    queue: VecDeque<Data<'source, Token>>,
}

impl<'a, Token: Logos<'a>> TokenReader<'a, Token> for Tokens<'a, Token> {
    fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>> {
        self.current.as_ref().map(|(a, _, _)| a.as_ref())
    }

    fn get_current_span(&self) -> Option<Span> {
        self.current.as_ref().map(|(_, a, _)| *a)
    }

    fn current(&self) -> Option<RefData<'_, 'a, Token>> {
        self.current.as_ref().map(|(a, b, c)| (a.as_ref(), *b, *c))
    }

    fn current_slice(&self) -> Option<&'a <Token::Source as Source>::Slice> {
        self.current.as_ref().map(|(_, _, a)| *a)
    }

    fn extras(&self) -> &Token::Extras {
        &self.lexer.extras
    }

    fn advance(&mut self) {
        self.current = self.queue.pop_front().or_else(|| {
            self.lexer.next().map(|token| {
                (
                    token,
                    Span::new(self.file, self.lexer.span()),
                    self.lexer.slice(),
                )
            })
        })
    }

    fn is_eoi(&self) -> bool {
        self.current.is_none()
    }
}

impl<'a, Token: Logos<'a>> Tokens<'a, Token> {
    pub fn new_with_file(file: FileId, lexer: Lexer<'a, Token>) -> Self {
        let mut s = Self {
            file,
            lexer,
            current: None,
            queue: Default::default(),
        };
        s.advance();
        s
    }

    fn add_to_queue(&mut self, tokens: VecDeque<Data<'a, Token>>) {
        self.queue.extend(tokens.into_iter())
    }
}
impl<'a, Token: Logos<'a, Extras = FileId>> Tokens<'a, Token> {
    pub fn new(lexer: Lexer<'a, Token>) -> Self {
        Self::new_with_file(lexer.extras, lexer)
    }
}

impl<'a, Token: Logos<'a, Extras = FileId>> From<Lexer<'a, Token>> for Tokens<'a, Token> {
    fn from(value: Lexer<'a, Token>) -> Self {
        Self::new(value)
    }
}

impl<'a, Token: Logos<'a> + Clone> Transactionable for Tokens<'a, Token> {
    type Transaction<'t> = TokensTransaction<'a, 't, Token, Self>
    where
        Self: 't;

    fn transaction(&mut self) -> Self::Transaction<'_> {
        TokensTransaction {
            current: self.current.clone(),
            parent: self,
            add_to_parent_queue: Self::add_to_queue,
            in_trans: false,
            queue: Default::default(),
            transaction: Default::default(),
        }
    }
}
pub struct TokensTransaction<
    'source,
    'parent,
    Token: Logos<'source>,
    Parent: TokenReader<'source, Token>,
> {
    parent: &'parent mut Parent,
    current: Option<Data<'source, Token>>,
    queue: VecDeque<Data<'source, Token>>,
    transaction: VecDeque<Data<'source, Token>>,
    add_to_parent_queue: fn(&'parent mut Parent, VecDeque<Data<'source, Token>>),
    in_trans: bool,
}

impl<'source, 'parent, Token, Parent> TokenReader<'source, Token>
    for TokensTransaction<'source, 'parent, Token, Parent>
where
    Token: Logos<'source> + Clone,
    Parent: TokenReader<'source, Token>,
    Token::Error: Clone,
{
    fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>> {
        self.current.as_ref().map(|(a, _, _)| a.as_ref())
    }

    fn get_current_span(&self) -> Option<Span> {
        self.current.as_ref().map(|(_, a, _)| *a)
    }

    fn current(&self) -> Option<RefData<'_, 'source, Token>> {
        self.current.as_ref().map(|(a, b, c)| (a.as_ref(), *b, *c))
    }

    fn current_slice(&self) -> Option<&'source <Token::Source as Source>::Slice> {
        self.current.as_ref().map(|(_, _, a)| *a)
    }

    fn extras(&self) -> &Token::Extras {
        self.parent.extras()
    }

    fn advance(&mut self) {
        self.current = self.queue.pop_front().or_else(|| {
            self.parent.advance();
            self.parent.current().map(|(token, span, slice)| {
                let data = (token.map(Clone::clone).map_err(Clone::clone), span, slice);
                if !self.in_trans {
                    self.transaction.push_back(data.clone());
                }
                data
            })
        });
    }

    fn is_eoi(&self) -> bool {
        self.parent.is_eoi()
    }
}

impl<'source, 'parent, Token, Parent> Transactionable
    for TokensTransaction<'source, 'parent, Token, Parent>
where
    Token: Logos<'source> + Clone,
    Parent: TokenReader<'source, Token>,
{
    type Transaction<'t> = TokensTransaction<'source, 't, Token, Self>
    where
        Self: 't;

    fn transaction(&mut self) -> Self::Transaction<'_> {
        self.in_trans = true;
        TokensTransaction {
            current: self.current.clone(),
            parent: self,
            add_to_parent_queue: Self::add_to_queue,
            in_trans: false,
            queue: Default::default(),
            transaction: Default::default(),
        }
    }
}

impl<'source, 'parent, Token, Parent> Transaction
    for TokensTransaction<'source, 'parent, Token, Parent>
where
    Token: Logos<'source> + Clone,
    Parent: TokenReader<'source, Token>,
{
    fn commit(self) {
        (self.add_to_parent_queue)(self.parent, self.queue)
    }

    fn discard(mut self) {
        self.transaction.extend(self.queue.into_iter());
        (self.add_to_parent_queue)(self.parent, self.transaction)
    }
}

impl<'source, 'parent, Token, Parent> TokensTransaction<'source, 'parent, Token, Parent>
where
    Token: Logos<'source>,
    Parent: TokenReader<'source, Token>,
{
    fn add_to_queue(&mut self, tokens: VecDeque<Data<'source, Token>>) {
        self.in_trans = false;
        self.queue.extend(tokens.into_iter())
    }
}
