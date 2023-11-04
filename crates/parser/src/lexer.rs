use logos::{Logos, SpannedIter};

mod token;

use span::Spanned;
pub use token::Token;

#[cfg(test)] mod tests;

pub type SpannedToken<Tok, Loc, E> = Result<(Loc, Tok, Loc), E>;

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
	type Item = SpannedToken<Token, usize, Spanned<LexError>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|(token, span)| {
			token
				.map(|t| (span.start, t, span.end))
				.map_err(|e| Spanned::new(span, e))
		})
	}
}
