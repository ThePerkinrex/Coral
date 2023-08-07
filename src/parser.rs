use thiserror::Error;

use crate::{
    ast::{Block, Expression, Identifier, Item, Statement, Type},
    error::Context,
    lexer::{tokens::TokenReader, Token},
    span::{Span, Spanned},
};

fn parse_token<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
    expected: Token,
) -> Result<Span, C::Error> {
    let r = match tokens.current() {
        None => Err(context.message(ParseError::UnexpectedEOI)),
        Some((token, span)) => match token {
            Ok(x) if x == &expected => Ok(span),
            Ok(&token) => Err(context.message(ParseError::UnexpectedTokenWithExpectation {
                found: span.spanned(token),
                expected,
            })),
            Err(()) => Err(context.message(ParseError::InvalidToken(span))),
        },
    };
    tokens.advance();
    r
}

fn maybe_parse_token<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
    expected: Token,
) -> Result<Option<Span>, C::Error> {
    match tokens.current() {
        None => Err(context.message(ParseError::UnexpectedEOI)),
        Some((token, span)) => match token {
            Ok(x) if x == &expected => {
                tokens.advance();
                Ok(Some(span))
            }
            Ok(_) => Ok(None),
            Err(()) => {
                tokens.advance();
                Err(context.message(ParseError::InvalidToken(span)))
            }
        },
    }
}

pub fn parse_item<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
) -> Result<Spanned<Item<'source>>, C::Error> {
    context.context("item", |context| match tokens.current() {
        Some((token, span)) => match token {
            Ok(&token) => match token {
                Token::Fn => {
                    tokens.advance();
                    context.enter_context("fn");
                    let name = parse_identifier(tokens, context)?;
                    context.enter_context(format_args!("{}", name.data));
                    parse_token(tokens, context, Token::OpeningParen)?;
                    let mut arguments = Vec::new();
                    context.context("arguments", |context| -> Result<(), C::Error> {
                        while {
                            let closing = maybe_parse_token(tokens, context, Token::ClosingParen)?;
                            closing.is_none()
                        } {
                            let identifier = parse_identifier(tokens, context)?;

                            parse_token(tokens, context, Token::Colon)?;
                            let ty = parse_type(tokens, context)?;
                            arguments.push((identifier, ty));
                            if tokens.get_current_token() == Some(Ok(&Token::Comma)) {
                                tokens.advance()
                            } else {
                                break;
                            }
                        }
                        Ok(())
                    })?;
                    parse_token(tokens, context, Token::Colon)?;
                    let return_type = parse_type(tokens, context)?;
                    let body = parse_block(tokens, context)?;
                    context.exit_ctx();
                    context.exit_ctx();
                    Ok(span.spanned(Item::Function {
                        name,
                        arguments,
                        return_type,
                        body,
                    }))
                }
                _ => Err(context.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Err(()) => Err(context.message(ParseError::InvalidToken(span))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    })
}

pub fn parse_statement<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
) -> Result<Spanned<Statement<'source>>, C::Error> {
    context.context("statement", |context| match tokens.current() {
        Some((token, span)) => match token {
            Ok(_) => {
                let expr = parse_expression(tokens, context)?;
                let semicolon = maybe_parse_token(tokens, context, Token::Semicolon)?;
                if let Some(semicolon) = semicolon {
                    Ok(Span::from_ends(expr.span, semicolon)
                        .unwrap()
                        .spanned(Statement::Expr(expr)))
                } else {
                    Ok(expr.span.spanned(Statement::ReturnExpr(expr)))
                }
            }
            Err(()) => Err(context.message(ParseError::InvalidToken(span))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    })
}

pub fn parse_expression<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
) -> Result<Spanned<Expression<'source>>, C::Error> {
    context.context("expression", |context| match tokens.current() {
        Some((token, span)) => match token {
            Ok(&token) => match token {
                Token::Identifier => parse_identifier(tokens, context)
                    .map(|ident| span.spanned(Expression::Name(ident))),
                _ => Err(context.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Err(()) => Err(context.message(ParseError::InvalidToken(span))),
        },
        None => Err(context.message(ParseError::UnexpectedEOI)),
    })
}

pub fn parse_identifier<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
) -> Result<Spanned<Identifier<'source>>, C::Error> {
    context.context("identifier", |context| {
        let r = match tokens.current() {
            Some((Ok(&token), span)) => match token {
                Token::Identifier => Ok(span.spanned(tokens.current_slice().unwrap().into())),
                _ => Err(context.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Some((Err(()), span)) => Err(context.message(ParseError::InvalidToken(span))),
            None => Err(context.message(ParseError::UnexpectedEOI)),
        };
        tokens.advance();
        r
    })
}

pub fn parse_type<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
) -> Result<Spanned<Type<'source>>, C::Error> {
    context.context("type", |context| {
        let r = match tokens.current() {
            Some((Ok(&token), span)) => match token {
                Token::Identifier => Ok(span.spanned(tokens.current_slice().unwrap().into())),
                _ => Err(context.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Some((Err(()), span)) => Err(context.message(ParseError::InvalidToken(span))),
            None => Err(context.message(ParseError::UnexpectedEOI)),
        };
        tokens.advance();
        r
    })
}

pub fn parse_block<'source, C: Context>(
    tokens: &mut impl TokenReader<'source, Token>,
    context: &mut C,
) -> Result<Spanned<Block<'source>>, C::Error> {
    context.context("block", |context| {
        let start = parse_token(tokens, context, Token::OpeningBracket)?;
        let mut statements = Vec::new();
        let end = loop {
            if let Some(span) = maybe_parse_token(tokens, context, Token::ClosingBracket)? {
                break span;
            }
            let current_span = tokens.get_current_span();
            let statement = parse_statement(tokens, context);
            match statement {
                Ok(statement) => {
                    let return_stmnt = matches!(statement.data, Statement::ReturnExpr(_));
                    statements.push(statement);
                    if return_stmnt {
                        break parse_token(tokens, context, Token::ClosingBracket)?;
                    }
                }
                Err(err) => {
                    if tokens.get_current_span() == current_span {
                        return Err(err);
                    }
                }
            }
        };

        Ok(Span::from_ends(start, end).unwrap().spanned(statements))
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEOI,
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Spanned<Token>),
    #[error("Unexpected token: {found:?}; expected: {expected:?}")]
    UnexpectedTokenWithExpectation {
        found: Spanned<Token>,
        expected: Token,
    },
    #[error("Invalid token")]
    InvalidToken(Span),
}
