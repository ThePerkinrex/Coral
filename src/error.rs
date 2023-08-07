use thiserror::Error;

use crate::parser::ParseError;

pub trait Context {
    type Error;
    type Or<'t>: Or + 't
    where
        Self: 't;

    fn enter_context<S: std::fmt::Display>(&mut self, name: S);
    fn exit_ctx(&mut self);
    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error;
    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error>;
    fn or(&mut self) -> Self::Or<'_>;
    fn context<T, F: FnOnce(&mut Self) -> T, S: std::fmt::Display>(&mut self, name: S, f: F) -> T {
        self.enter_context(name);
        let r = f(self);
        self.exit_ctx();
        r
    }
}

pub trait Or {
    type OptionContext: Context;
    fn option(&self, name: &'static str) -> Self::OptionContext;
    fn accept(self, option: Self::OptionContext);
    fn discard<const N: usize>(self, options: [Self::OptionContext; N]);
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoralError {
    #[error(transparent)]
    ParserError(#[from] ParseError),
}

mod mock;
pub use mock::MockContext;

mod printing;
pub use printing::PrintingContext;
