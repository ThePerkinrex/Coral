use std::iter::Peekable;

use thiserror::Error;

use crate::{
    ast::{Expression, Identifier, Item, Statement, Type},
    error::Context,
    lexer::{tokens::TokenReader, Token},
    span::{Spanned, Span},
    FileArena,
};

pub fn parse_item<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
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
    tokens: &mut impl TokenReader<'source, Token>,
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
    tokens: &mut impl TokenReader<'source, Token>,
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
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Identifier<'source>>, C::Error> {
    match tokens.current() {
        Some((Ok(&token), span)) => match token {
            Token::Identifier => {
                Ok(span.spanned(tokens.current_slice().unwrap().into()))
            }
            _ => Err(context.message(ParseError::UnexpectedToken(span.spanned(token)))),
        },
        Some((Err(()), span)) => Err(context.message(ParseError::InvalidToken(span))),
        None => Err(context.message(ParseError::UnexpectedEOI)),
    }
}

pub fn parse_type<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
    files: &'source FileArena,
) -> Result<Spanned<Type<'source>>, C::Error> {
    match tokens.current() {
        Some((Ok(&token), span)) => match token {
            Token::Identifier => {
                // TODO advance it?
                Ok(span.spanned(tokens.current_slice().unwrap().into()))
            }
            _ => Err(context.message(ParseError::UnexpectedToken(span.spanned(token)))),
        },
        Some((Err(()), span)) => Err(context.message(ParseError::InvalidToken(span))),
        None => Err(context.message(ParseError::UnexpectedEOI)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEOI,
    #[error("Unexpected token")]
    UnexpectedToken(Spanned<Token>),
    #[error("Invalid token")]
    InvalidToken(Span),
}
