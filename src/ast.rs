use std::borrow::Cow;

use crate::span::Spanned;

pub enum Item<'a> {
    Function {
        name: Spanned<Identifier<'a>>,
        arguments: Vec<(Spanned<Identifier<'a>>, Spanned<Type<'a>>)>,
        return_type: Spanned<Type<'a>>,
        body: Spanned<Vec<Spanned<Statement<'a>>>>,
    },
}

pub enum Statement<'a> {
    Expr(Spanned<Expression<'a>>),
}

pub enum Expression<'a> {
    Name(Spanned<Identifier<'a>>),
}

pub struct Identifier<'a>(Cow<'a, str>);
impl<'a, S: Into<Cow<'a, str>>> From<S> for Identifier<'a> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
pub struct Type<'a>(Cow<'a, str>);
impl<'a, S: Into<Cow<'a, str>>> From<S> for Type<'a> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
