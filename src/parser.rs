use std::iter::Peekable;

use thiserror::Error;

use crate::{
    ast::{Expression, Identifier, Item, Statement, Type},
    error::Context,
    lexer::{SpannedToken, Token},
    span::Spanned,
    FileArena,
};

pub fn parse_item<'source, C: Context>(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken<'source, Token>>>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Item<'source>>, C::Error> {
    // match tokens.peek() {
    //     Some(span) => {
    //         let span: Span<_> = span.clone();
    //         match span.data {
    //             Ok(token) => match token {
    //                 Token::Fn => {
    //                     tokens.next();
    //                     let name = parse_identifier(tokens, context, files)?;
    //                     if matches!(tokens.next())
    //                     Ok(span.map(|_| Item::Function { name, arguments: (), return_type: (), body: () }))
    //                 }
    //                 _ => {
    //                     Err(context.message(ParseError::UnexpectedToken(span.copy_new_data(token))))
    //                 }
    //             },
    //             Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
    //         }
    //     }
    //     None => Err(context.message(ParseError::UnexpectedEOI)),
    // }
    todo!()
}

pub fn parse_statement<'source, C: Context>(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken<'source, Token>>>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Statement<'source>>, C::Error> {
    match tokens.peek() {
        Some(span) => match span.data {
            Ok(_) => {
                let expr = parse_expression(tokens, context, files)?;

                let other = expr.copy_new_data(());
                Ok(other.copy_new_data(Statement::Expr(expr)))
            }
            Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    }
}

pub fn parse_expression<'source, C: Context>(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken<'source, Token>>>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Expression<'source>>, C::Error> {
    match tokens.peek() {
        Some(span) => {
            let span: Spanned<_> = span.clone();
            match span.data {
                Ok(token) => match token {
                    Token::Identifier => parse_identifier(tokens, context, files)
                        .map(|ident| span.copy_new_data(Expression::Name(ident))),
                    _ => {
                        Err(context.message(ParseError::UnexpectedToken(span.copy_new_data(token))))
                    }
                },
                Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
            }
        }
        None => Err(context.message(ParseError::UnexpectedEOI)),
    }
}

pub fn parse_identifier<'source, C: Context>(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken<'source, Token>>>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Identifier<'source>>, C::Error> {
    let mut advance = 0;
    let res = match tokens.peek() {
        Some(span) => match span.data {
            Ok(token) => match token {
                Token::Identifier => {
                    advance = 1;
                    Ok(span.copy_new_data(span.get_slice(files).into()))
                }
                _ => Err(context.message(ParseError::UnexpectedToken(span.copy_new_data(token)))),
            },
            Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    };
    for _ in 0..advance {
        tokens.next();
    }
    res
}

pub fn parse_type<'source, C: Context>(
    tokens: &mut Peekable<impl Iterator<Item = SpannedToken<'source, Token>>>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Type<'source>>, C::Error> {
    let mut advance = 0;
    let res = match tokens.peek() {
        Some(span) => match span.data {
            Ok(token) => match token {
                Token::Identifier => {
                    advance = 1;
                    Ok(span.copy_new_data(span.get_slice(files).into()))
                }
                _ => Err(context.message(ParseError::UnexpectedToken(span.copy_new_data(token)))),
            },
            Err(()) => Err(context.message(ParseError::InvalidToken(span.copy_new_data(())))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    };
    for _ in 0..advance {
        tokens.next();
    }
    res
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEOI,
    #[error("Unexpected token")]
    UnexpectedToken(Spanned<Token>),
    #[error("Invalid token")]
    InvalidToken(Spanned<()>),
}
