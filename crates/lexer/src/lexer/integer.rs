use winnow::{ascii::dec_int, PResult, Parser};

use super::Stream;
use crate::Token;

pub fn integer(i: &mut Stream<'_>) -> PResult<Token> {
	dec_int
		.map(Token::Integer)
		.parse_next(i)
}
