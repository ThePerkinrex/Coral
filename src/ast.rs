use std::borrow::Cow;

use crate::span::Spanned;

#[derive(Debug)]
pub enum Item<'a> {
    Function {
        name: Spanned<Identifier<'a>>,
        arguments: Vec<(Spanned<Identifier<'a>>, Spanned<Type<'a>>)>,
        return_type: Spanned<Type<'a>>,
        body: Spanned<Block<'a>>,
    },
}

pub type Block<'a> = Vec<Spanned<Statement<'a>>>;

#[derive(Debug)]
pub enum Statement<'a> {
    Expr(Spanned<Expression<'a>>),
    ReturnExpr(Spanned<Expression<'a>>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    Name(Spanned<Identifier<'a>>),
}

#[derive(Debug)]
pub struct Identifier<'a>(Cow<'a, str>);
impl<'a, S: Into<Cow<'a, str>>> From<S> for Identifier<'a> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Type<'a>(Cow<'a, str>);
impl<'a, S: Into<Cow<'a, str>>> From<S> for Type<'a> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
