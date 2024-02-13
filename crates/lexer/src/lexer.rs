use std::ops::Range;

use span::{Span, Spanned};
use winnow::{
	combinator::{alt, cut_err, eof},
	Located, PResult, Parser,
};

use crate::{LexError, Token};

mod comment;
mod identifier;
mod integer;
mod keyword;
mod string;
mod whitespace;

use comment::{block_comment, line_comment};
use identifier::identifier;
use integer::integer;
use keyword::keyword;
use string::string;
use whitespace::whitespace;

type Stream<'a> = Located<&'a str>;

pub fn token(i: &mut Stream<'_>) -> PResult<Token> {
	alt((
		whitespace,
		line_comment,
		block_comment,
		identifier,
		keyword,
		integer,
		string,
	))
	.parse_next(i)
}

pub type SpannedToken<Tok, Loc, E> = Result<(Loc, Tok, Loc), E>;

pub struct SpannedLexer<'a>(Located<&'a str>);

impl<'a> SpannedLexer<'a> {
	pub fn new(code: &'a str) -> Self { Self(Located::new(code)) }
}

impl<'a> Iterator for SpannedLexer<'a> {
	type Item = SpannedToken<Token, usize, Spanned<LexError>>;

	fn next(&mut self) -> Option<Self::Item> {
		match alt((eof.value(None), token.map(Some)))
			.with_span()
			.parse_next(&mut self.0)
		{
			Ok((Some(tok), Range { start, end })) =>
				Some(Ok((start, tok, end))),
			Ok((None, _)) => None,
			Err(err) => {
				let err = err.into_inner();
				println!("{:?}", err);
				Some(Err(Spanned::new(
					Span::DUMMY_SPAN,
					LexError::InvalidToken,
				)))
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn count_tokens() {
		let code = "a + 2";
		let lexer = SpannedLexer::new(code);

		assert_eq!(lexer.count(), 5);
	}

	#[test]
	fn match_longer_first() {
		let code = "usethis";
		let mut lexer = SpannedLexer::new(code);

		assert_eq!(
			lexer.next(),
			Some(Ok((
				0,
				Token::Identifier("usethis".to_owned()),
				7
			)))
		);
	}
}
