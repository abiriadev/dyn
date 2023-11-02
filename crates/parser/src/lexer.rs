use logos::{Logos, SpannedIter};

mod token;

pub use token::Token;

#[cfg(test)] mod tests;

pub type Spanned<Tok, Loc, E> = Result<(Loc, Tok, Loc), E>;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexError {
	InvalidIdentifier,

	#[default]
	InvalidToken,
}

pub struct SpannedLexer<'a>(SpannedIter<'a, Token>);

impl<'a> SpannedLexer<'a> {
	pub fn new(code: &'a str) -> Self { Self(Token::lexer(code).spanned()) }
}

impl<'a> Iterator for SpannedLexer<'a> {
	type Item = Spanned<Token, usize, LexError>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0
			.next()
			.map(|(token, span)| token.map(|t| (span.start, t, span.end)))
	}
}
