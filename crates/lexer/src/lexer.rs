use winnow::{combinator::alt, Located, PResult, Parser};

use crate::Token;

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
