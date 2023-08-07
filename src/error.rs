use thiserror::Error;

use crate::parser::ParseError;

pub trait Context {
    type Error;
    type Or<'t>: Or + 't
    where
        Self: 't;

    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error;
    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error>;
    fn or(&mut self) -> Self::Or<'_>;
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
