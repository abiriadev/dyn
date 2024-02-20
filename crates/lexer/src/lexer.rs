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

pub struct LexerConfig {
	pub ignore_whitespace: bool,
	pub asi: bool,
}

pub struct SpannedLexer<'a> {
	code: Located<&'a str>,
	last: Option<TokenKind>,
	config: LexerConfig,
}

impl<'a> SpannedLexer<'a> {
	pub fn new(code: &'a str, config: LexerConfig) -> Self {
		Self {
			code: Located::new(code),
			last: None,
			config,
		}
	}
}

impl<'a> Iterator for SpannedLexer<'a> {
	type Item = SpannedToken;

	fn next(&mut self) -> Option<Self::Item> {
		use winnow::stream::Stream;

		loop {
			let start = self.code.checkpoint();

			break match alt((eof.value(None), token.map(Some)))
				.with_span()
				.parse_next(&mut self.code)
			{
				Ok((Some(tok), span)) => {
					if self.config.ignore_whitespace && tok == Token::Whitespace
					{
						continue;
					}
					if self.config.asi
						&& tok == Token::NewLine && !matches!(
						self.last,
						Some(
							TokenKind::Nil
								| TokenKind::True | TokenKind::False
								| TokenKind::Integer | TokenKind::String
								| TokenKind::Identifier
								| TokenKind::RightParenthesis
								| TokenKind::RightBrace | TokenKind::RightBracket
						)
					) {
						continue;
					}

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
			Some(SpannedToken {
				token: Ok(Token::Identifier("usethis".to_owned())),
				span: (0..7).into()
			})
		);
	}

	#[test]
	fn match_shorter_first() {
		let code = "use";
		let mut lexer = SpannedLexer::new(code);

		assert_eq!(
			lexer.next(),
			Some(SpannedToken {
				token: Ok(Token::Use),
				span: (0..3).into()
			})
		);
	}
}
