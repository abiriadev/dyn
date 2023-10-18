use pretty_assertions::assert_eq;

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
	assert_eq!(
		Token::lexer("\n")
			.spanned()
			.collect::<Vec<_>>(),
		[(Ok(Token::NewLine), 0..1)]
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

#[test]
fn lex_string() {
	assert_eq!(
		Token::lexer(r#""""#)
			.spanned()
			.collect::<Vec<_>>(),
		[(Ok(Token::String("".to_owned())), 0..2)]
	);
}

#[test]
fn lex_identifier() {
	assert_eq!(
		Token::lexer("_")
			.spanned()
			.collect::<Vec<_>>(),
		[(
			Ok(Token::Identifier("_".to_owned())),
			0..1
		)]
	);
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
