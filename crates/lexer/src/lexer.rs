use winnow::{
	combinator::{alt, eof},
	stream::Offset,
	Located, PResult, Parser,
};

use crate::{token::TokenKind, LexError, SpannedToken, Token};

mod comment;
mod identifier;
mod integer;
mod punctuation;
mod string;
mod whitespace;

use comment::{block_comment, line_comment};
use identifier::identifier;
use integer::integer;
use punctuation::punctuation;
use string::string;
use whitespace::whitespace;

type Stream<'a> = Located<&'a str>;

fn token(i: &mut Stream<'_>) -> PResult<Token> {
	alt((
		whitespace,
		line_comment,
		block_comment,
		identifier,
		punctuation,
		integer,
		string,
	))
	.parse_next(i)
}

pub struct SpannedLexer<'a> {
	code: Located<&'a str>,
	last: Option<TokenKind>,
}

impl<'a> SpannedLexer<'a> {
	pub fn new(code: &'a str) -> Self {
		Self {
			code: Located::new(code),
			last: None,
		}
	}
}

impl<'a> Iterator for SpannedLexer<'a> {
	type Item = SpannedToken;

	fn next(&mut self) -> Option<Self::Item> {
		use winnow::stream::Stream;

		let start = self.code.checkpoint();

		match alt((eof.value(None), token.map(Some)))
			.with_span()
			.parse_next(&mut self.code)
		{
			Ok((Some(tok), span)) => {
				self.last = Some(TokenKind::from(&tok));

				Some(SpannedToken::new(tok, span.into()))
			},
			Ok((None, _)) => None,
			Err(_) => {
				// let err = err.into_inner();
				// println!("{:?}", err);
				let offset = self.code.offset_from(&start);
				Some(SpannedToken::new_err(
					LexError::InvalidToken,
					(offset..offset + 1).into(),
				))
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

	#[test]
	fn match_shorter_first() {
		let code = "use";
		let mut lexer = SpannedLexer::new(code);

		assert_eq!(
			lexer.next(),
			Some(Ok((0, Token::Use, 3)))
		);
	}
}
