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

#[cfg(test)]
mod tests {
	use indoc::indoc;
	use winnow::Located;

	use super::*;

	#[test]
	fn should_allow_nested_block_comment() {
		let mut code = Located::new(indoc! {"
            /*
                Outer comment
                /*
                    Inner comment
                */
            */
        "});

		assert_eq!(
			block_comment.parse_next(&mut code),
			Ok(Token::BlockComment)
		);
	}
}
