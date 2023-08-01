use crate::{
    ast::{Expression, Item, Statement},
    error::{CoralResult, ErrorGatherer},
    lexer::{SpannedToken, Token},
    span::Span, FileArena,
};

pub fn parse_item<'source>(
    tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>,
) -> CoralResult<Span<Item<'source>>, ()> {
    todo!()
}

pub fn parse_statement<'source>(
    tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>,
	files: &'source FileArena
) -> CoralResult<Span<Statement<'source>>, ParseError> {
    let mut result = ErrorGatherer::new();
	let mut tokens = tokens.peekable();
    match tokens.peek() {
        Some(span) => match span.data {
            Ok(_) => {
				if let Some(expr) = result.gather(parse_expression(&mut tokens, files)) {
					let other = expr.copy_new_data(());
					return result.result(other.copy_new_data(Statement::Expr(expr)))
				}else{
					return result.unrecoverable()
				}
			},
            Err(()) => result.add(ParseError::InvalidToken(span.copy_new_data(()))),
        },
        None => result.add(ParseError::UnexpectedEOI),
    }

    result.unrecoverable()
}

pub fn parse_expression<'source>(
    tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>,
	files: &'source FileArena
) -> CoralResult<Span<Expression<'source>>, ParseError> {
    let mut result = ErrorGatherer::new();
    match tokens.next() {
        Some(span) => match span.data {
            Ok(token) => match token {
                Token::Identifier => {
					return result.result(span.copy_new_data(Expression::Name(span.copy_new_data(span.get_slice(files).into()))))
				},
                _ => result.add(ParseError::UnexpectedToken(span.copy_new_data(token))),
            },
            Err(()) => result.add(ParseError::InvalidToken(span.copy_new_data(()))),
        },
        None => result.add(ParseError::UnexpectedEOI),
    }

    result.unrecoverable()
}

pub enum ParseError {
    UnexpectedEOI,
    UnexpectedToken(Span<Token>),
    InvalidToken(Span<()>),
}
