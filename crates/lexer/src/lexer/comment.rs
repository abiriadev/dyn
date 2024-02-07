use winnow::{
	ascii::till_line_ending,
	combinator::{alt, delimited, not},
	PResult, Parser,
};

use super::I;
use crate::Token;

pub fn line_comment(i: &mut I<'_>) -> PResult<Token> {
	("//", till_line_ending)
		.value(Token::LineComment)
		.parse_next(i)
}

pub fn block_comment(i: &mut I<'_>) -> PResult<Token> {
	delimited(
		"/*",
		alt((
			not(alt(("/*", "*/"))),
			block_comment.void(),
		)),
		"*/",
	)
	.value(Token::BlockComment)
	.parse_next(i)
}
