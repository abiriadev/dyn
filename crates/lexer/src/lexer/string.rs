use winnow::{
	ascii::escaped_transform,
	combinator::{alt, delimited},
	token::take_till,
	PResult, Parser,
};

use super::Stream;
use crate::{token::QuoteKind, QuotedString, Token};

pub fn string(i: &mut Stream<'_>) -> PResult<Token> {
	alt((string_single, string_double)).parse_next(i)
}

pub fn string_single(i: &mut Stream<'_>) -> PResult<Token> {
	delimited(
		'\'',
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
		),
		'\'',
	)
	.map(|content| {
		Token::String(QuotedString {
			content,
			quote: QuoteKind::Single,
		})
	})
	.parse_next(i)
}

pub fn string_double(i: &mut Stream<'_>) -> PResult<Token> {
	delimited(
		'"',
		escaped_transform(
			take_till(0.., ['"', '\\', '{', '}']),
			'\\',
			alt((
				"\\".value("\\"),
				"\"".value("\""),
				"{".value("{"),
				"}".value("}"),
				"n".value("\n"),
				"r".value("\r"),
				"t".value("\t"),
			)),
		),
		'"',
	)
	.map(|content| {
		Token::String(QuotedString {
			content,
			quote: QuoteKind::Double,
		})
	})
	.parse_next(i)
}

pub fn template_string_leading_fragment(i: &mut Stream<'_>) -> PResult<Token> {
	delimited(
		'"',
		escaped_transform(
			take_till(0.., ['"', '\\', '{', '}', '#']),
			'\\',
			alt((
				"\\".value("\\"),
				"\"".value("\""),
				"{".value("{"),
				"}".value("}"),
				"n".value("\n"),
				"r".value("\r"),
				"t".value("\t"),
			)),
		),
		"#{",
	)
	.map(Token::TemplateStringLeadingFragment)
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
			Ok(Token::String(QuotedString {
				content: "John Doe".to_owned(),
				quote: QuoteKind::Single
			}))
		);
	}

	#[test]
	fn should_parse_double_quoted_string() {
		assert_eq!(
			string(&mut Located::new(r#""John Doe""#)),
			Ok(Token::String(QuotedString {
				content: "John Doe".to_owned(),
				quote: QuoteKind::Double
			}))
		);
	}
}
