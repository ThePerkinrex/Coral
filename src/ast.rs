use std::borrow::Cow;

use crate::span::Span;

pub enum Item<'a> {
	Function {
		name: Span<Identifier<'a>>,
		arguments: Vec<(Span<Identifier<'a>>, Span<Type<'a>>)>,
		return_type: Span<Type<'a>>,
		body: Span<Vec<Span<Statement<'a>>>>
	}
}

pub enum Statement<'a> {
	Expr(Span<Expression<'a>>)
}

pub enum Expression<'a> {
	Name(Span<Identifier<'a>>),
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