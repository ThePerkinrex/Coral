use crate::{lexer::{SpannedToken, Token}, ast::{Item, Expression, Statement}, error::CoralResult};

pub fn parse_item<'source>(tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>) -> CoralResult<Item, ()> {
	todo!()

}

pub fn parse_statement<'source>(tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>) -> CoralResult<Statement, ()> {
	todo!()
}

pub fn parse_expression<'source>(tokens: &mut impl Iterator<Item = SpannedToken<'source, Token>>) -> CoralResult<Expression, ()> {
	todo!()
}