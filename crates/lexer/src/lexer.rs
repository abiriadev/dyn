use std::ops::Range;

use span::{Span, Spanned};
use winnow::{
	combinator::{alt, cut_err},
	error::ErrMode,
	Located, PResult, Parser,
};

use crate::{LexError, Token};

mod comment;
mod identifier;
mod keyword;
mod string;
mod whitespace;

use comment::{block_comment, line_comment};
use identifier::identifier;
use keyword::keyword;
use string::string;
use whitespace::whitespace;

type Stream<'a> = Located<&'a str>;

pub fn token(i: &mut Stream<'_>) -> PResult<Token> {
	alt((
		whitespace,
		line_comment,
		block_comment,
		keyword,
		identifier,
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
		match cut_err(token)
			.with_span()
			.parse_next(&mut self.0)
		{
			Ok((tok, Range { start, end })) => Some(Ok((start, tok, end))),
			Err(err) => {
				let err = err.into_inner();
				Some(Err(Spanned::new(
					Span::DUMMY_SPAN,
					LexError::InvalidToken,
				)))
			},
		}
	}
}
