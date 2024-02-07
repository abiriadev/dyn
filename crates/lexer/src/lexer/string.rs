use winnow::{
	ascii::escaped_transform,
	combinator::{alt, todo},
	token::none_of,
	PResult, Parser,
};

use super::Stream;
use crate::Token;

pub fn string(i: &mut Stream<'_>) -> PResult<Token> {
	alt((string_single, string_double)).parse_next(i)
}

pub fn string_single(i: &mut Stream<'_>) -> PResult<Token> {
	escaped_transform(
		none_of(['\'', '\\']),
		'\\',
		alt((
			'\\'.value('\\'),
			'\''.value('\''),
			'n'.value('\n'),
			'r'.value('\r'),
			't'.value('\t'),
		)),
	)
	.parse_next(i)
}

pub fn string_double(i: &mut Stream<'_>) -> PResult<Token> {
	escaped_transform(
		none_of(['"', '\\', '{', '}']),
		'\\',
		alt((
			'\\'.value('\\'),
			'"'.value('"'),
			'{'.value('{'),
			'}'.value('}'),
			'n'.value('\n'),
			'r'.value('\r'),
			't'.value('\t'),
		)),
	)
	.parse_next(i)
}
