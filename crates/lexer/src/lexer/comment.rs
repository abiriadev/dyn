use winnow::{ascii::till_line_ending, PResult, Parser};

use super::I;
use crate::Token;

pub fn line_comment(i: &mut I<'_>) -> PResult<Token> {
	("//", till_line_ending)
		.value(Token::LineComment)
		.parse_next(i)
}
