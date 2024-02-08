use std::marker::PhantomData;

use thiserror::Error;

mod lexer;
mod token;

use span::Spanned;
pub use token::{QuotedString, Token};

#[cfg(test)] mod tests;

pub type SpannedToken<Tok, Loc, E> = Result<(Loc, Tok, Loc), E>;

#[derive(Default, Debug, Clone, PartialEq, Error)]
pub enum LexError {
	#[error("InvalidIdentifier")]
	InvalidIdentifier,

	#[default]
	#[error("InvalidToken")]
	InvalidToken,
}

pub struct SpannedLexer<'a>(PhantomData<&'a ()>);

impl<'a> SpannedLexer<'a> {
	pub fn new(code: &'a str) -> Self { todo!() }
}

impl<'a> Iterator for SpannedLexer<'a> {
	type Item = SpannedToken<Token, usize, Spanned<LexError>>;

	fn next(&mut self) -> Option<Self::Item> {
		// self.0.next().map(|(token, span)| {
		// 	token
		// 		.map(|t| (span.start, t, span.end))
		// 		.map_err(|e| Spanned::new(span, e))
		// })
		todo!()
	}
}
