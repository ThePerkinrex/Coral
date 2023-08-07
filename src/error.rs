use thiserror::Error;

use crate::parser::ParseError;

pub trait Context {
    type Error;

    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error;
    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoralError {
    #[error(transparent)]
    ParserError(#[from] ParseError)
}

#[derive(Debug, Default)]
pub struct MockContext;

impl Context for MockContext {
    type Error = ();

    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error {
        
    }

    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error> {
        Err(())
    }
}
