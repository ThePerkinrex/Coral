use logos::{Lexer, Logos};

use crate::{span::Span, fs::FileId};

pub trait TokenReader<'a, Token: Logos<'a>> {
	fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>>;
	fn get_current_span(&self) -> Option<&Span>;
	fn extras(&self) -> &Token::Extras;
	fn advance(&mut self);
	fn is_eoi(&self) -> bool;
}

pub struct Tokens<'source, Token: Logos<'source>> {
	file: FileId,
	lexer: Lexer<'source, Token>,
	current: Option<(Result<Token, Token::Error>, Span)>,
}

impl<'a, Token: Logos<'a>> TokenReader<'a, Token> for Tokens<'a, Token> {
    fn get_current_token(&self) -> Option<Result<&Token, &Token::Error>> {
        self.current.as_ref().map(|(a, _)| a.as_ref())
    }

    fn get_current_span(&self) -> Option<&Span> {
        self.current.as_ref().map(|(_, a)| a)
    }

	fn extras(&self) -> &Token::Extras {
		&self.lexer.extras
	}

    fn advance(&mut self) {
        self.current = self.lexer.next().map(|token| (token, Span::new(self.file, self.lexer.span())))
    }

    fn is_eoi(&self) -> bool {
        self.current.is_none()
    }
}

impl<'a, Token: Logos<'a>> Tokens<'a, Token> {
	pub fn new_with_file(file: FileId, lexer: Lexer<'a, Token>) -> Self {
		let mut s = Self {file, lexer, current: None};
		s.advance();
		s
	}

	
}
impl<'a, Token: Logos<'a, Extras = FileId>> Tokens<'a, Token> {
	pub fn new(lexer: Lexer<'a, Token>) -> Self {
		Self::new_with_file(lexer.extras, lexer)
	}
}
