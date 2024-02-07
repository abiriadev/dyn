use winnow::{
	ascii::till_line_ending,
	combinator::{peek, preceded, repeat_till},
	token::{any, take},
	PResult, Parser,
};

use super::Stream;
use crate::Token;

pub fn line_comment(i: &mut Stream<'_>) -> PResult<Token> {
	("//", till_line_ending)
		.value(Token::LineComment)
		.parse_next(i)
}

pub fn block_comment(i: &mut Stream<'_>) -> PResult<Token> {
	preceded(
		"/*",
		repeat_till::<_, _, (), _, _, _, _>(0.., block_comment_inner, "*/"),
	)
	.value(Token::BlockComment)
	.parse_next(i)
}

fn block_comment_inner(i: &mut Stream<'_>) -> PResult<()> {
	if peek(take(2usize)).parse_next(i)? == "/*" {
		block_comment.void().parse_next(i)
	} else {
		any.void().parse_next(i)
	}
}

#[cfg(test)]
mod tests {
	use indoc::indoc;
	use pretty_assertions::assert_eq;
	use winnow::{combinator::trace, Located};

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
			trace("block_comment", block_comment).parse_next(&mut code),
			Ok(Token::BlockComment)
		);
	}

	#[test]
	fn should_not_allow_unfinished_nested_block_comment() {
		let mut code = Located::new(indoc! {"
		    /*
		        Outer comment
		        /*
		            Inner comment
		        */
		"});

		assert!(trace("block_comment", block_comment)
			.parse_next(&mut code)
			.is_err());
	}
}
