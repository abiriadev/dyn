use indoc::indoc;
use pretty_assertions::{assert_eq, assert_ne};

use super::*;

mod lex_tag_tokens {
	use logos::Logos;

	use super::{assert_eq, Token};

	#[test]
	fn lex_plus() {
		assert_eq!(
			Token::lexer("+")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Plus), 0..1)]
		);
	}

	#[test]
	fn lex_minus() {
		assert_eq!(
			Token::lexer("-")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Minus), 0..1)]
		);
	}

	#[test]
	fn lex_asterisk() {
		assert_eq!(
			Token::lexer("*")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Asterisk), 0..1)]
		);
	}

	#[test]
	fn lex_slash() {
		assert_eq!(
			Token::lexer("/")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Slash), 0..1)]
		);
	}

	#[test]
	fn lex_percent() {
		assert_eq!(
			Token::lexer("%")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Percent), 0..1)]
		);
	}

	#[test]
	fn lex_assign() {
		assert_eq!(
			Token::lexer("=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Assign), 0..1)]
		);
	}

	#[test]
	fn lex_plus_assign() {
		assert_eq!(
			Token::lexer("+=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::PlusAssign), 0..2)]
		);
	}

	#[test]
	fn lex_minus_assign() {
		assert_eq!(
			Token::lexer("-=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::MinusAssign), 0..2)]
		);
	}

	#[test]
	fn lex_asterisk_assign() {
		assert_eq!(
			Token::lexer("*=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::AsteriskAssign), 0..2)]
		);
	}

	#[test]
	fn lex_slash_assign() {
		assert_eq!(
			Token::lexer("/=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::SlashAssign), 0..2)]
		);
	}

	#[test]
	fn lex_percent_assign() {
		assert_eq!(
			Token::lexer("%=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::PercentAssign), 0..2)]
		);
	}

	#[test]
	fn lex_equal() {
		assert_eq!(
			Token::lexer("==")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Equal), 0..2)]
		);
	}

	#[test]
	fn lex_not_equal() {
		assert_eq!(
			Token::lexer("!=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::NotEqual), 0..2)]
		);
	}

	#[test]
	fn lex_left_angled_bracket() {
		assert_eq!(
			Token::lexer("<")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LeftAngledBracket), 0..1)]
		);
	}

	#[test]
	fn lex_right_angled_bracket() {
		assert_eq!(
			Token::lexer(">")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::RightAngledBracket), 0..1)]
		);
	}

	#[test]
	fn lex_less_than_equal() {
		assert_eq!(
			Token::lexer("<=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LessThanEqual), 0..2)]
		);
	}

	#[test]
	fn lex_greater_than_equal() {
		assert_eq!(
			Token::lexer(">=")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::GreaterThanEqual), 0..2)]
		);
	}

	#[test]
	fn lex_double_and() {
		assert_eq!(
			Token::lexer("&&")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::DoubleAnd), 0..2)]
		);
	}

	#[test]
	fn lex_double_pipe() {
		assert_eq!(
			Token::lexer("||")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::DoublePipe), 0..2)]
		);
	}

	#[test]
	fn lex_left_paranthesis() {
		assert_eq!(
			Token::lexer("(")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LeftParenthesis), 0..1)]
		);
	}

	#[test]
	fn lex_right_paranthesis() {
		assert_eq!(
			Token::lexer(")")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::RightParenthesis), 0..1)]
		);
	}

	#[test]
	fn lex_left_brace() {
		assert_eq!(
			Token::lexer("{")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LeftBrace), 0..1)]
		);
	}

	#[test]
	fn lex_right_brace() {
		assert_eq!(
			Token::lexer("}")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::RightBrace), 0..1)]
		);
	}

	#[test]
	fn lex_left_bracket() {
		assert_eq!(
			Token::lexer("[")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LeftBracket), 0..1)]
		);
	}

	#[test]
	fn lex_right_bracket() {
		assert_eq!(
			Token::lexer("]")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::RightBracket), 0..1)]
		);
	}

	#[test]
	fn lex_bang() {
		assert_eq!(
			Token::lexer("!")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Bang), 0..1)]
		);
	}

	#[test]
	fn lex_dot() {
		assert_eq!(
			Token::lexer(".")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Dot), 0..1)]
		);
	}

	#[test]
	fn lex_comma() {
		assert_eq!(
			Token::lexer(",")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Comma), 0..1)]
		);
	}

	#[test]
	fn lex_pipe() {
		assert_eq!(
			Token::lexer("|")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Pipe), 0..1)]
		);
	}

	#[test]
	fn lex_at() {
		assert_eq!(
			Token::lexer("@")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::At), 0..1)]
		);
	}

	#[test]
	fn lex_arrow() {
		assert_eq!(
			Token::lexer("->")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Arrow), 0..2)]
		);
	}

	#[test]
	fn lex_nil() {
		assert_eq!(
			Token::lexer("nil")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Nil), 0..3)]
		);
	}

	#[test]
	fn lex_true() {
		assert_eq!(
			Token::lexer("true")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::True), 0..4)]
		);
	}

	#[test]
	fn lex_false() {
		assert_eq!(
			Token::lexer("false")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::False), 0..5)]
		);
	}

	#[test]
	fn lex_panic() {
		assert_eq!(
			Token::lexer("panic")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Panic), 0..5)]
		);
	}

	#[test]
	fn lex_assert() {
		assert_eq!(
			Token::lexer("assert")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Assert), 0..6)]
		);
	}

	#[test]
	fn lex_let() {
		assert_eq!(
			Token::lexer("let")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Let), 0..3)]
		);
	}

	#[test]
	fn lex_let_mut() {
		assert_eq!(
			Token::lexer("let!")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LetMut), 0..4)]
		);
	}

	#[test]
	fn lex_if() {
		assert_eq!(
			Token::lexer("if")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::If), 0..2)]
		);
	}

	#[test]
	fn lex_else() {
		assert_eq!(
			Token::lexer("else")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Else), 0..4)]
		);
	}

	#[test]
	fn lex_iter() {
		assert_eq!(
			Token::lexer("iter")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Iter), 0..4)]
		);
	}

	#[test]
	fn lex_of() {
		assert_eq!(
			Token::lexer("of")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Of), 0..2)]
		);
	}

	#[test]
	fn lex_return() {
		assert_eq!(
			Token::lexer("return")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Return), 0..6)]
		);
	}

	#[test]
	fn lex_break() {
		assert_eq!(
			Token::lexer("break")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Break), 0..5)]
		);
	}

	#[test]
	fn lex_continue() {
		assert_eq!(
			Token::lexer("continue")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Continue), 0..8)]
		);
	}

	#[test]
	fn lex_import() {
		assert_eq!(
			Token::lexer("import")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Import), 0..6)]
		);
	}

	#[test]
	fn lex_export() {
		assert_eq!(
			Token::lexer("export")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::Export), 0..6)]
		);
	}
}

#[test]
fn lex_newline() {
	// FIX: ASI
	assert_ne!(
		Token::lexer("\n")
			.spanned()
			.collect::<Vec<_>>(),
		[(Ok(Token::NewLine), 0..1)]
	);
}

#[test]
fn lex_comment_between_newlines() {
	assert_eq!(
		Token::lexer("\n/* comment */\n")
			.spanned()
			.collect::<Vec<_>>(),
		[
			// (Ok(Token::NewLine), 0..1), fix: ASI
			(Ok(Token::BlockCommnet), 1..14),
			// (Ok(Token::NewLine), 14..15)
		]
	);

	assert_eq!(
		Token::lexer(indoc! {"

			// comment
		"})
		.spanned()
		.collect::<Vec<_>>(),
		[
			// (Ok(Token::NewLine), 0..1),
			(Ok(Token::LineComment), 1..11),
			// (Ok(Token::NewLine), 11..12)
		]
	);
}

#[test]
#[ignore]
fn sequential_newlines_should_be_parsed_as_separate_tokens() {
	assert_eq!(
		Token::lexer("\n\n\n")
			.spanned()
			.collect::<Vec<_>>(),
		[
			(Ok(Token::NewLine), 0..1),
			(Ok(Token::NewLine), 1..2),
			(Ok(Token::NewLine), 2..3)
		]
	);
}

mod lex_comments {
	use logos::Logos;

	use super::{assert_eq, Token};

	#[test]
	fn lex_line_comment() {
		assert_eq!(
			Token::lexer("//")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LineComment), 0..2)]
		);
	}

	#[test]
	fn multiple_line_comments_should_be_allowed() {
		assert_eq!(
			Token::lexer("// abc // def")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::LineComment), 0..13)]
		);
	}

	#[test]
	fn each_line_comment_should_be_parsed_as_separate_tokens() {
		assert_eq!(
			Token::lexer("// abc\n// def")
				.spanned()
				.collect::<Vec<_>>(),
			[
				// not a single token of two lines long.
				(Ok(Token::LineComment), 0..6),
				// (Ok(Token::NewLine), 6..7), reason: ASI
				(Ok(Token::LineComment), 7..13)
			]
		);
	}

	#[test]
	fn lex_block_comment() {
		assert_eq!(
			Token::lexer("/**/")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::BlockCommnet), 0..4)]
		);

		assert_eq!(
			Token::lexer("/* */")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::BlockCommnet), 0..5)]
		);
	}

	#[test]
	fn block_comment_should_be_able_to_contain_newline() {
		assert_eq!(
			Token::lexer("/*\n\n\n*/")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::BlockCommnet), 0..7)]
		);
	}

	#[test]
	fn nested_block_comment_should_be_allowed() {
		assert_eq!(
			Token::lexer("/*/**/*/")
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::BlockCommnet), 0..8),]
		);
	}
}

#[test]
fn lex_integer() {
	assert_eq!(
		Token::lexer("0")
			.spanned()
			.collect::<Vec<_>>(),
		[(Ok(Token::Integer(0)), 0..1)]
	);
}

#[test]
fn minus_zero_should_be_valid_integer() {
	assert_eq!(
		Token::lexer("-0")
			.spanned()
			.collect::<Vec<_>>(),
		[(Ok(Token::Integer(0)), 0..2)]
	);
}

mod lex_string {
	use logos::Logos;

	use super::{assert_eq, LexError, Token};

	#[test]
	fn lex_string() {
		assert_eq!(
			Token::lexer(r#""""#)
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::String("".to_owned())), 0..2)]
		);

		assert_eq!(
			Token::lexer(r#"''"#)
				.spanned()
				.collect::<Vec<_>>(),
			[(Ok(Token::String("".to_owned())), 0..2)]
		);
	}

	#[test]
	fn strings_should_have_same_content_regardless_of_quotes_used() {
		assert_eq!(
			Token::lexer(r#""abc""#)
				.spanned()
				.collect::<Vec<_>>(),
			Token::lexer(r#"'abc'"#)
				.spanned()
				.collect::<Vec<_>>(),
		);
	}

	#[test]
	fn strings_should_use_quote_escape_depending_on_the_quotes_used() {
		assert_eq!(
			Token::lexer(r#""\"'""#)
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::String(r#"\"'"#.to_owned())),
				0..5,
			)]
		);

		assert_eq!(
			Token::lexer(r#"'\'"'"#)
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::String(r#"\'""#.to_owned())),
				0..5,
			)]
		);
	}

	#[test]
	fn string_should_be_closed_with_the_same_quote_used_to_open() {
		assert_eq!(
			Token::lexer(r#""'"#)
				.spanned()
				.collect::<Vec<_>>(),
			[(Err(LexError::InvalidToken), 0..2)]
		);

		assert_eq!(
			Token::lexer(r#"'""#)
				.spanned()
				.collect::<Vec<_>>(),
			[(Err(LexError::InvalidToken), 0..2)]
		);
	}
}

mod lex_identifier {
	use logos::Logos;

	use super::{assert_eq, assert_ne, LexError, Token};

	#[test]
	fn lex_identifier() {
		assert_eq!(
			Token::lexer("a")
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::Identifier("a".to_owned())),
				0..1
			)]
		);
	}

	#[test]
	fn underscores_should_be_valid_identifiers() {
		assert_eq!(
			Token::lexer("_")
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::Identifier("_".to_owned())),
				0..1
			)]
		);

		assert_eq!(
			Token::lexer("__")
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::Identifier("__".to_owned())),
				0..2
			)]
		);

		assert_eq!(
			Token::lexer("_a")
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::Identifier("_a".to_owned())),
				0..2
			)]
		);

		assert_eq!(
			Token::lexer("a_")
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::Identifier("a_".to_owned())),
				0..2
			)]
		);
	}

	#[test]
	fn identifiers_should_be_case_sensitive() {
		let (i1, i1_span) = Token::lexer("asdf")
			.spanned()
			.next()
			.unwrap();

		let (i2, i2_span) = Token::lexer("ASDF")
			.spanned()
			.next()
			.unwrap();

		// they must have the same length
		assert_eq!(i1_span, i2_span);

		// but they should have different values
		assert_ne!(i1, i2);
	}

	#[test]
	fn identifier_must_not_start_with_number() {
		assert_eq!(
			Token::lexer("123a")
				.spanned()
				.collect::<Vec<_>>(),
			[(Err(LexError::InvalidIndentifier), 0..4)]
		);
	}

	#[test]
	fn ifif() {
		assert_eq!(
			Token::lexer("ifif")
				.spanned()
				.collect::<Vec<_>>(),
			[(
				Ok(Token::Identifier("ifif".to_owned())),
				0..4
			)]
		);
	}
}

#[test]
fn lex_math_expr() {
	assert_eq!(
		//            01234567890123456789
		Token::lexer("1 + 2 * 3 % (4 - 5)")
			.spanned()
			.collect::<Vec<_>>(),
		[
			(Ok(Token::Integer(1)), 0..1),
			(Ok(Token::Plus), 2..3),
			(Ok(Token::Integer(2)), 4..5),
			(Ok(Token::Asterisk), 6..7),
			(Ok(Token::Integer(3)), 8..9),
			(Ok(Token::Percent), 10..11),
			(Ok(Token::LeftParenthesis), 12..13),
			(Ok(Token::Integer(4)), 13..14),
			(Ok(Token::Minus), 15..16),
			(Ok(Token::Integer(5)), 17..18),
			(Ok(Token::RightParenthesis), 18..19),
		]
	)
}

mod asi {
	use indoc::indoc;
	use logos::Logos;

	use super::{assert_eq, Token};
	use crate::lexer::token::TokenKind;

	#[test]
	fn last_token() {
		let mut lexer = Token::lexer("let a = 1 + 2");

		assert_eq!(lexer.extras, None);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Let);

		assert_eq!(lexer.extras, Some(TokenKind::Let));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Identifier("a".to_owned()));

		assert_eq!(
			lexer.extras,
			Some(TokenKind::Identifier)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Assign);

		assert_eq!(lexer.extras, Some(TokenKind::Assign));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Integer(1));

		assert_eq!(lexer.extras, Some(TokenKind::Integer));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Plus);

		assert_eq!(lexer.extras, Some(TokenKind::Plus));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Integer(2));

		assert_eq!(lexer.extras, Some(TokenKind::Integer));
		let tok = lexer.next();
		assert_eq!(tok, None);
	}

	#[test]
	fn must_insert_semicolon_after_literal() {
		let mut lexer = Token::lexer(indoc! {"
			a = 1 + 2
			+ 3
		"});

		assert_eq!(lexer.extras, None);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Identifier("a".to_owned()));

		assert_eq!(
			lexer.extras,
			Some(TokenKind::Identifier)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Assign);

		assert_eq!(lexer.extras, Some(TokenKind::Assign));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Integer(1));

		assert_eq!(lexer.extras, Some(TokenKind::Integer));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Plus);

		assert_eq!(lexer.extras, Some(TokenKind::Plus));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Integer(2));

		assert_eq!(lexer.extras, Some(TokenKind::Integer));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::NewLine);

		assert_eq!(lexer.extras, Some(TokenKind::NewLine));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Plus);

		assert_eq!(lexer.extras, Some(TokenKind::Plus));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Integer(3));

		assert_eq!(lexer.extras, Some(TokenKind::Integer));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::NewLine);

		assert_eq!(lexer.extras, Some(TokenKind::NewLine));
		let tok = lexer.next();
		assert_eq!(tok, None);
	}

	#[test]
	fn function_call_span_multiple_lines() {
		let mut lexer = Token::lexer(indoc! {"
			print(
				a,
				b + 10
			)
		"});

		assert_eq!(lexer.extras, None);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(
			tok,
			Token::Identifier("print".to_owned())
		);

		assert_eq!(
			lexer.extras,
			Some(TokenKind::Identifier)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::LeftParenthesis);

		assert_eq!(
			lexer.extras,
			Some(TokenKind::LeftParenthesis)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Identifier("a".to_owned()));

		assert_eq!(
			lexer.extras,
			Some(TokenKind::Identifier)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Comma);

		assert_eq!(lexer.extras, Some(TokenKind::Comma));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Identifier("b".to_owned()));

		assert_eq!(
			lexer.extras,
			Some(TokenKind::Identifier)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Plus);

		assert_eq!(lexer.extras, Some(TokenKind::Plus));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::Integer(10));

		assert_eq!(lexer.extras, Some(TokenKind::Integer));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::NewLine);

		assert_eq!(lexer.extras, Some(TokenKind::NewLine));
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::RightParenthesis);

		assert_eq!(
			lexer.extras,
			Some(TokenKind::RightParenthesis)
		);
		let tok = lexer.next().unwrap().unwrap();
		assert_eq!(tok, Token::NewLine);

		assert_eq!(lexer.extras, Some(TokenKind::NewLine));
		let tok = lexer.next();
		assert_eq!(tok, None);
	}
}
