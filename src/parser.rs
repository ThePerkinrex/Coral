use thiserror::Error;

use crate::{
    ast::{Block, Expression, Identifier, Item, Statement, Type},
    error::Context,
    lexer::{tokens::TokenReader, Token},
    span::{Span, Spanned},
    transaction::ParserState,
};

use self::context::ContextName;

mod context;

fn parse_token<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
    expected: Token,
) -> Result<Span, C::Error> {
    let r = match state.current() {
        None => Err(state.message(ParseError::UnexpectedEOI)),
        Some((token, span, _)) => match token {
            Ok(x) if x == &expected => Ok(span),
            Ok(&token) => Err(state.message(ParseError::UnexpectedTokenWithExpectation {
                found: span.spanned(token),
                expected,
            })),
            Err(()) => Err(state.message(ParseError::InvalidToken(span))),
        },
    };
    state.advance();
    r
}

fn maybe_parse_token<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
    expected: Token,
) -> Result<Option<Span>, C::Error> {
    match state.current() {
        None => Err(state.message(ParseError::UnexpectedEOI)),
        Some((token, span, _)) => match token {
            Ok(x) if x == &expected => {
                state.advance();
                Ok(Some(span))
            }
            Ok(_) => Ok(None),
            Err(()) => {
                state.advance();
                Err(state.message(ParseError::InvalidToken(span)))
            }
        },
    }
}

pub fn parse_item<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
) -> Result<Spanned<Item<'source>>, C::Error> {
    state.context(ContextName::Item, |state| match state.current() {
        Some((token, span, _)) => match token {
            Ok(&token) => match token {
                Token::Fn => {
                    state.advance();
                    state.enter_context(ContextName::Fn);
                    let name = parse_identifier(state)?;
                    state.enter_context(ContextName::FnName(name.data.to_string()));
                    parse_token(state, Token::OpeningParen)?;
                    let mut arguments = Vec::new();
                    state.context(ContextName::Arguments, |state| -> Result<(), C::Error> {
                        while {
                            let closing = maybe_parse_token(state, Token::ClosingParen)?;
                            closing.is_none()
                        } {
                            let identifier = parse_identifier(state)?;

                            parse_token(state, Token::Colon)?;
                            let ty = parse_type(state)?;
                            arguments.push((identifier, ty));
                            if state.get_current_token() == Some(Ok(&Token::Comma)) {
                                state.advance()
                            } else {
                                break;
                            }
                        }
                        Ok(())
                    })?;
                    parse_token(state, Token::Colon)?;
                    let return_type = parse_type(state)?;
                    let body = parse_block(state)?;
                    state.exit_ctx();
                    state.exit_ctx();
                    Ok(span.spanned(Item::Function {
                        name,
                        arguments,
                        return_type,
                        body,
                    }))
                }
                _ => Err(state.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Err(()) => Err(state.message(ParseError::InvalidToken(span))),
        },
        None => Err(state.message(ParseError::UnexpectedEOI)),
    })
}

pub fn parse_statement<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
) -> Result<Spanned<Statement<'source>>, C::Error> {
    state.context(
        ContextName::Statement,
        |state: &mut ParserState<C, T>| match state.current() {
            Some((token, span, _)) => match token {
                Ok(_) => {
                    let expr = parse_expression(state)?;
                    let semicolon = maybe_parse_token(state, Token::Semicolon)?;
                    if let Some(semicolon) = semicolon {
                        Ok(Span::from_ends(expr.span, semicolon)
                            .unwrap()
                            .spanned(Statement::Expr(expr)))
                    } else {
                        Ok(expr.span.spanned(Statement::ReturnExpr(expr)))
                    }
                }
                Err(()) => Err(state.message(ParseError::InvalidToken(span))),
            },
            None => Err(state.message(ParseError::UnexpectedEOI)),
        },
    )
}

pub fn parse_expression<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
) -> Result<Spanned<Expression<'source>>, C::Error> {
    state.context(ContextName::Expression, |state| match state.current() {
        Some((token, span, _)) => match token {
            Ok(&token) => match token {
                Token::Identifier => {
                    parse_identifier(state).map(|ident| span.spanned(Expression::Name(ident)))
                }
                _ => Err(state.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Err(()) => Err(state.message(ParseError::InvalidToken(span))),
        },
        None => Err(state.message(ParseError::UnexpectedEOI)),
    })
}

pub fn parse_identifier<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
) -> Result<Spanned<Identifier<'source>>, C::Error> {
    state.context(ContextName::Identifier, |state| {
        let r = match state.current() {
            Some((Ok(&token), span, slice)) => match token {
                Token::Identifier => Ok(span.spanned(slice.into())),
                _ => Err(state.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Some((Err(()), span, _)) => Err(state.message(ParseError::InvalidToken(span))),
            None => Err(state.message(ParseError::UnexpectedEOI)),
        };
        state.advance();
        r
    })
}

pub fn parse_type<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
) -> Result<Spanned<Type<'source>>, C::Error> {
    state.context(ContextName::Type, |state| {
        let r = match state.current() {
            Some((Ok(&token), span, slice)) => match token {
                Token::Identifier => Ok(span.spanned(slice.into())),
                _ => Err(state.message(ParseError::UnexpectedToken(span.spanned(token)))),
            },
            Some((Err(()), span, _)) => Err(state.message(ParseError::InvalidToken(span))),
            None => Err(state.message(ParseError::UnexpectedEOI)),
        };
        state.advance();
        r
    })
}

pub fn parse_block<'source, C: Context<ContextName>, T: TokenReader<'source, Token>>(
    state: &mut ParserState<C, T>,
) -> Result<Spanned<Block<'source>>, C::Error> {
    state.context(ContextName::Block, |state| {
        let start = parse_token(state, Token::OpeningBracket)?;
        let mut statements = Vec::new();
        let end = loop {
            if let Some(span) = maybe_parse_token(state, Token::ClosingBracket)? {
                break span;
            }
            let current_span = state.get_current_span();
            let statement = parse_statement(state);
            match statement {
                Ok(statement) => {
                    let return_stmnt = matches!(statement.data, Statement::ReturnExpr(_));
                    statements.push(statement);
                    if return_stmnt {
                        break parse_token(state, Token::ClosingBracket)?;
                    }
                }
                Err(err) => {
                    if state.get_current_span() == current_span {
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
