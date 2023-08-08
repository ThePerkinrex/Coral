use thiserror::Error;

use crate::parser::ParseError;

pub trait ContextName {}

pub trait Context<C: ContextName> {
    type Error;

    fn enter_context(&mut self, name: C);
    fn exit_ctx(&mut self);
    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error;
    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error>;
    fn context<T, F: FnOnce(&mut Self) -> T>(&mut self, name: C, f: F) -> T {
        self.enter_context(name);
        let r = f(self);
        self.exit_ctx();
        r
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoralError {
    #[error(transparent)]
    ParserError(#[from] ParseError),
}

mod printing_context;
pub use printing_context::PrintingContext;
