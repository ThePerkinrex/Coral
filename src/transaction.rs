use logos::Logos;

use crate::{
    error::{Context, ContextName},
    lexer::tokens::{RefData, TokenReader},
};

pub trait Transactionable {
    type Transaction<'t>: Transaction
    where
        Self: 't;
    fn transaction(&mut self) -> Self::Transaction<'_>;
}

pub trait Transaction: Transactionable {
    fn commit(self);
    fn discard(self);
}

pub struct ParserState<C, T> {
    ctx: C,
    tokens: T,
}

impl<C, T> ParserState<C, T> {
    pub const fn new(ctx: C, tokens: T) -> Self {
        Self { ctx, tokens }
    }
}

impl<C, T> Transactionable for ParserState<C, T>
where
    C: Transactionable,
    T: Transactionable,
{
    type Transaction<'t> = ParserState<C::Transaction<'t>, T::Transaction<'t>>
    where
        Self: 't;

    fn transaction(&mut self) -> Self::Transaction<'_> {
        ParserState {
            ctx: self.ctx.transaction(),
            tokens: self.tokens.transaction(),
        }
    }
}

impl<C, T> Transaction for ParserState<C, T>
where
    C: Transaction,
    T: Transaction,
{
    fn commit(self) {
        self.ctx.commit();
        self.tokens.commit();
    }

    fn discard(self) {
        self.ctx.discard();
        self.tokens.discard();
    }
}

impl<C, T, CName: ContextName> Context<CName> for ParserState<C, T>
where
    C: Context<CName>,
{
    type Error = C::Error;

    fn enter_context(&mut self, name: CName) {
        self.ctx.enter_context(name)
    }

    fn exit_ctx(&mut self) {
        self.ctx.exit_ctx()
    }

    fn message<M: Into<crate::error::CoralError>>(&mut self, msg: M) -> Self::Error {
        self.ctx.message(msg)
    }

    fn report<O, E: Into<crate::error::CoralError>>(
        &mut self,
        res: Result<O, E>,
    ) -> Result<O, Self::Error> {
        self.ctx.report(res)
    }
}

impl<'source, C, T, Token> TokenReader<'source, Token> for ParserState<C, T>
where
    T: TokenReader<'source, Token>,
    Token: Logos<'source>,
{
    fn get_current_token(&self) -> Option<Result<&Token, &<Token as Logos<'source>>::Error>> {
        self.tokens.get_current_token()
    }

    fn get_current_span(&self) -> Option<crate::span::Span> {
        self.tokens.get_current_span()
    }

    fn current(&self) -> Option<RefData<'_, 'source, Token>> {
        self.tokens.current()
    }

    fn current_slice(
        &self,
    ) -> Option<&'source <<Token as Logos<'source>>::Source as logos::Source>::Slice> {
        self.tokens.current_slice()
    }

    fn extras(&self) -> &<Token as Logos<'source>>::Extras {
        self.tokens.extras()
    }

    fn advance(&mut self) {
        self.tokens.advance()
    }

    fn is_eoi(&self) -> bool {
        self.tokens.is_eoi()
    }
}
