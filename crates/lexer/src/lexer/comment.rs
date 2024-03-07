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
	use crate::{lexer::LexerConfig, SpannedLexer};

	#[test]
	fn lex_line_comment() {
		assert_eq!(
			line_comment.parse_next(&mut Located::new("//")),
			Ok(Token::LineComment)
		);
	}

	#[test]
	fn multiple_line_comments_should_be_allowed() {
		assert_eq!(
			line_comment.parse_next(&mut Located::new("// abc // def")),
			Ok(Token::LineComment)
		);
	}

	#[test]
	fn each_line_comment_should_be_parsed_as_separate_tokens() {
		let mut src =
			SpannedLexer::new("// abc\n// def", LexerConfig::default());

		assert_eq!(
			src.next().unwrap().token,
			Ok(Token::LineComment)
		);
		// assert_eq!(
		// 	src.next().unwrap().token,
		// 	Ok(Token::NewLine)
		// );
		assert_eq!(
			src.next().unwrap().token,
			Ok(Token::LineComment)
		);
	}

	#[test]
	fn lex_block_comment() {
		assert_eq!(
			block_comment.parse_next(&mut Located::new("/**/")),
			Ok(Token::BlockComment)
		);

		assert_eq!(
			block_comment.parse_next(&mut Located::new("/* */")),
			Ok(Token::BlockComment)
		);
	}

	#[test]
	fn block_comment_should_be_able_to_contain_newline() {
		assert_eq!(
			block_comment.parse_next(&mut Located::new("/*\n\n\n*/")),
			Ok(Token::BlockComment)
		);
	}

	#[test]
	fn should_allow_nested_block_comment() {
		assert_eq!(
			block_comment.parse_next(&mut Located::new("/*/**/*/")),
			Ok(Token::BlockComment)
		);

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
