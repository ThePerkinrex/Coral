use logos::{Lexer, Logos, Source};

use crate::{fs::FileId, span::Span};

pub trait TokenReader<'source, Token: Logos<'source>> {
    fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>>;
    fn get_current_span(&self) -> Option<Span>;
    fn current(&self) -> Option<(Result<&Token, &Token::Error>, Span)>;
    fn current_slice(&self) -> Option<&'source <Token::Source as Source>::Slice>;
    fn extras(&self) -> &Token::Extras;
    fn advance(&mut self);
    fn is_eoi(&self) -> bool;
}

pub struct Tokens<'source, Token: Logos<'source>> {
    file: FileId,
    lexer: Lexer<'source, Token>,
    current: Option<(Result<Token, Token::Error>, Span)>,
}

impl<'a, Token: Logos<'a>> TokenReader<'a, Token> for Tokens<'a, Token> {
    fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>> {
        self.current.as_ref().map(|(a, _)| a.as_ref())
    }

    fn get_current_span(&self) -> Option<Span> {
        self.current.as_ref().map(|(_, a)| *a)
    }

    fn current(&self) -> Option<(Result<&Token, &Token::Error>, Span)> {
        self.current.as_ref().map(|(a, b)| (a.as_ref(), *b))
    }

    fn current_slice(&self) -> Option<&'a <Token::Source as Source>::Slice> {
        if self.is_eoi() {
            None
        } else {
            Some(self.lexer.slice())
        }
    }

    fn extras(&self) -> &Token::Extras {
        &self.lexer.extras
    }

    fn advance(&mut self) {
        self.current = self
            .lexer
            .next()
            .map(|token| (token, Span::new(self.file, self.lexer.span())))
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
        };
        s.advance();
        s
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
