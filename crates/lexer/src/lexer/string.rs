use winnow::{
	combinator::{alt, todo},
	token::{none_of, one_of, take_while},
	PResult, Parser,
};

use super::Stream;
use crate::Token;

pub fn string(i: &mut Stream<'_>) -> PResult<Token> {
	alt((string_single, string_double)).parse_next(i)
}

pub fn string_single(i: &mut Stream<'_>) -> PResult<Token> { todo(i) }

pub fn string_double(i: &mut Stream<'_>) -> PResult<Token> { todo(i) }
