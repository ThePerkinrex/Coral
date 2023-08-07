use std::ops::{Deref, DerefMut, Index};

use crate::{
    fs::{File, FileId},
    span::Spanned,
};
use logos::{Lexer, Logos, SpannedIter};

pub mod tokens;

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+", extras = FileId)] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,

    #[token("(")]
    OpeningParen,
    #[token(")")]
    ClosingParen,
    #[token("{")]
    OpeningBracket,
    #[token("}")]
    ClosingBracket,
    #[token("[")]
    OpeningSqBracket,
    #[token("]")]
    ClosingSqBracket,

    #[token("=")]
    Assignment,

    #[regex("\\+|-|\\*|/|%|&&|\\|\\||==|!=|>|<|>=|<=")]
    Operator,

    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    PathSep,

    #[regex("[_a-zA-Z][_0-9a-zA-z]*")]
    Identifier,
    #[regex("#[_a-zA-Z][_0-9a-zA-z]*")]
    IntrinsicIdentifier,

    #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())]
    IntegerLiteral(i128),
}

impl Token {
    pub fn lexer_from_file<A>(arena: &A, id: FileId) -> Lexer<'_, Self>
    where
        A: Index<FileId, Output = File>,
    {
        Self::lexer_with_extras(&arena[id].contents, id)
    }
}

#[cfg(test)]
mod test {
    use id_arena::Arena;

    use crate::fs::File;

    use super::Token;

    macro_rules! ok_or_err {
        (Ok) => {};
        (Err) => {};
        ($a:ident) => {
            compile_error!("Only Ok or Err are accepted")
        };
    }
    macro_rules! assert_tokens {
		($iter_spanned:ident) => {
			assert_eq!($iter_spanned.next(), None)
		};
		($iter_spanned:ident, $($ok_or_err:ident($data:expr, $range:expr)),+) => {
			{
				$({
					ok_or_err!($ok_or_err);
					assert_eq!($iter_spanned.next(), Some(($ok_or_err($data), $range)));
				};)+

				assert_eq!($iter_spanned.next(), None);
			}
		};
	}

    #[test]
    fn test_all() {
        use Token::*;
        let mut arena: Arena<File> = Arena::new();
        let f_b = arena.alloc(File {
            name: "b".into(),
            contents: "fn main() {hello == b && a >= c; #a != 1; let d = 2; std::yes() }".into(),
        });
        let lexer = Token::lexer_from_file(&arena, f_b);
        let mut spanned = lexer.spanned();
        assert_tokens!(
            spanned,
            Ok(Fn, 0..2),
            Ok(Identifier, 3..7),
            Ok(OpeningParen, 7..8),
            Ok(ClosingParen, 8..9),
            Ok(OpeningBracket, 10..11),
            Ok(Identifier, 11..16),
            Ok(Operator, 17..19),
            Ok(Identifier, 20..21),
            Ok(Operator, 22..24),
            Ok(Identifier, 25..26),
            Ok(Operator, 27..29),
            Ok(Identifier, 30..31),
            Ok(Semicolon, 31..32),
            Ok(IntrinsicIdentifier, 33..35),
            Ok(Operator, 36..38),
            Ok(IntegerLiteral(1), 39..40),
            Ok(Semicolon, 40..41),
            Ok(Let, 42..45),
            Ok(Identifier, 46..47),
            Ok(Assignment, 48..49),
            Ok(IntegerLiteral(2), 50..51),
            Ok(Semicolon, 51..52),
            Ok(Identifier, 53..56),
            Ok(PathSep, 56..58),
            Ok(Identifier, 58..61),
            Ok(OpeningParen, 61..62),
            Ok(ClosingParen, 62..63),
            Ok(ClosingBracket, 64..65)
        );
    }
}

pub struct SpannedIterExt<'source, Token: Logos<'source>>(SpannedIter<'source, Token>, FileId);

impl<'source, Token: Logos<'source>> Deref for SpannedIterExt<'source, Token> {
    type Target = Lexer<'source, Token>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'source, Token: Logos<'source>> DerefMut for SpannedIterExt<'source, Token> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

pub type SpannedToken<'source, Token> = Spanned<Result<Token, <Token as Logos<'source>>::Error>>;

impl<'source, Token: Logos<'source>> Iterator for SpannedIterExt<'source, Token> {
    type Item = SpannedToken<'source, Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(token, range)| Spanned::new(self.1, range, token))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'source, Token: Logos<'source>> SpannedIterExt<'source, Token> {
    pub const fn new(iter: SpannedIter<'source, Token>, id: FileId) -> Self {
        Self(iter, id)
    }
}

impl<'source, Token: Logos<'source, Extras = FileId>> From<SpannedIter<'source, Token>>
    for SpannedIterExt<'source, Token>
{
    fn from(iter: SpannedIter<'source, Token>) -> Self {
        let extras = iter.extras;
        Self::new(iter, extras)
    }
}
