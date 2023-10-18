use logos::{Logos, SpannedIter};

use self::token::Token;

mod token;

#[cfg(test)] mod tests;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexError {
	InvalidIndentifier,

	#[default]
	InvalidToken,
}

pub struct SpannedLexer<'a>(SpannedIter<'a, Token>);

impl<'a> SpannedLexer<'a> {
	pub fn new(code: &'a str) -> Self { Self(Token::lexer(code).spanned()) }
}
