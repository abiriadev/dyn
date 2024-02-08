use winnow::{
	ascii::escaped_transform, combinator::alt, token::take_till, PResult,
	Parser,
};

use super::Stream;
use crate::Token;

pub fn string(i: &mut Stream<'_>) -> PResult<Token> {
	alt((string_single, string_double)).parse_next(i)
}

pub fn string_single(i: &mut Stream<'_>) -> PResult<Token> {
	escaped_transform(
		take_till(0.., ['\'', '\\']),
		'\\',
		alt((
			"\\".value("\\"),
			"'".value("'"),
			"n".value("\n"),
			"r".value("\r"),
			"t".value("\t"),
		)),
	)
	.parse_next(i)
}

pub fn string_double(i: &mut Stream<'_>) -> PResult<Token> {
	escaped_transform(
		take_till(0.., ['\'', '\\', '{', '}']),
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

#[cfg(test)]
mod tests {
	use winnow::Located;

	use super::*;

	#[test]
	fn should_parse_single_quoted_string() {
		assert_eq!(
			string(&mut Located::new("'John Doe'")),
			Ok(todo!())
		);
	}
}
