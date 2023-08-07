use thiserror::Error;

use crate::{
    ast::{Expression, Item, Statement},
    lexer::{SpannedToken, Token},
    span::Span, FileArena, error::Context,
};

pub fn parse_item<'source, C: Context>(
    tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>,
    context: &mut C,
) -> Result<Span<Item<'source>>, C::Error> {
    todo!()
}

pub fn parse_statement<'source, C: Context>(
    tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>,
    context: &mut C,
	files: &'source FileArena
) -> Result<Span<Statement<'source>>, C::Error> {
	let mut tokens = tokens.peekable();
    match tokens.peek() {
        Some(span) => match span.data {
            Ok(_) => {
                let expr = parse_expression(&mut tokens, context, files)?;
				
                let other = expr.copy_new_data(());
                Ok(other.copy_new_data(Statement::Expr(expr)))
			},
            Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    }
}

pub fn parse_expression<'source, C: Context>(
    tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>,
    context: &mut C,
	files: &'source FileArena
) -> Result<Span<Expression<'source>>, C::Error> {
    match tokens.next() {
        Some(span) => match span.data {
            Ok(token) => match token {
                Token::Identifier => {
					Ok(span.copy_new_data(Expression::Name(span.copy_new_data(span.get_slice(files).into()))))
				},
                _ => Err(context.message(ParseError::UnexpectedToken(span.copy_new_data(token)))),
            },
            Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEOI,
    #[error("Unexpected token")]
    UnexpectedToken(Span<Token>),
    #[error("Invalid token")]
    InvalidToken(Span<()>),
}
