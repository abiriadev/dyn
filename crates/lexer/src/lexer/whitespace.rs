use winnow::{token::take_while, PResult, Parser};

use super::Stream;
use crate::Token;

pub fn whitespace(i: &mut Stream<'_>) -> PResult<Token> {
	take_while(1.., [' ', '\t'])
		.value(Token::Whitespace)
		.parse_next(i)
}
