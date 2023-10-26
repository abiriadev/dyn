use logos::{Filter, Lexer, Logos};
use strum::EnumDiscriminants;

use super::LexError;
use crate::macros::save_token;

fn lex_string(lex: &mut Lexer<Token>) -> String {
	lex.extras = Some(TokenKind::String);
	let sl = lex.slice();
	sl[1..sl.len() - 1].to_owned()
}

fn asi(lex: &mut Lexer<Token>) -> Filter<()> {
	let res = match lex.extras {
		Some(
			// an identifier
			| TokenKind::Identifier

			// literals
			| TokenKind::Integer | TokenKind::String

			// operators and delimiters
			| TokenKind::RightAngledBracket
			| TokenKind::RightParenthesis
			| TokenKind::RightBrace
			| TokenKind::RightBracket,
		) => Filter::Emit(()),
		_ => Filter::Skip,
	};
	lex.extras = Some(TokenKind::NewLine);
	res
}

#[derive(Debug, Clone, PartialEq, Logos, EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
#[logos(
	skip r"[ \t]+",
	error = LexError,
	extras = Option<TokenKind>
)]
pub enum Token {
	// Elementary arithmetics
	#[token("+", save_token!(TokenKind::Plus))]
	Plus,

	#[token("-", save_token!(TokenKind::Minus))]
	Minus,

	#[token("*", save_token!(TokenKind::Asterisk))]
	Asterisk,

	#[token("/", save_token!(TokenKind::Slash))]
	Slash,

	#[token("%", save_token!(TokenKind::Percent))]
	Percent,

	// Assignment operators
	#[token("=", save_token!(TokenKind::Assign))]
	Assign,

	#[token("+=", save_token!(TokenKind::PlusAssign))]
	PlusAssign,

	#[token("-=", save_token!(TokenKind::MinusAssign))]
	MinusAssign,

	#[token("*=", save_token!(TokenKind::AsteriskAssign))]
	AsteriskAssign,

	#[token("/=", save_token!(TokenKind::SlashAssign))]
	SlashAssign,

	#[token("%=", save_token!(TokenKind::PercentAssign))]
	PercentAssign,

	// Comparison operators
	#[token("==", save_token!(TokenKind::Equal))]
	Equal,

	#[token("!=", save_token!(TokenKind::NotEqual))]
	NotEqual,

	#[token("<", save_token!(TokenKind::LeftAngledBracket))]
	LeftAngledBracket,

	#[token(">", save_token!(TokenKind::RightAngledBracket))]
	RightAngledBracket,

	#[token("<=", save_token!(TokenKind::LessThanEqual))]
	LessThanEqual,

	#[token(">=", save_token!(TokenKind::GreaterThanEqual))]
	GreaterThanEqual,

	// Boolean operators
	#[token("&&", save_token!(TokenKind::DoubleAnd))]
	DoubleAnd,

	#[token("||", save_token!(TokenKind::DoublePipe))]
	DoublePipe,

	// Parantheses
	#[token("(", save_token!(TokenKind::LeftParenthesis))]
	LeftParenthesis,

	#[token(")", save_token!(TokenKind::RightParenthesis))]
	RightParenthesis,

	#[token("{", save_token!(TokenKind::LeftBrace))]
	LeftBrace,

	#[token("}", save_token!(TokenKind::RightBrace))]
	RightBrace,

	#[token("[", save_token!(TokenKind::LeftBracket))]
	LeftBracket,

	#[token("]", save_token!(TokenKind::RightBracket))]
	RightBracket,

	// etc
	#[token("!", save_token!(TokenKind::Bang))]
	Bang,

	#[token(".", save_token!(TokenKind::Dot))]
	Dot,

	#[token(",", save_token!(TokenKind::Comma))]
	Comma,

	#[token("|", save_token!(TokenKind::Pipe))]
	Pipe,

	#[token("@", save_token!(TokenKind::At))]
	At,

	#[token("->", save_token!(TokenKind::Arrow))]
	Arrow,

	// token literals
	#[token("nil", save_token!(TokenKind::Nil))]
	Nil,

	#[token("true", save_token!(TokenKind::True))]
	True,

	#[token("false", save_token!(TokenKind::False))]
	False,

	// keywords
	#[token("panic", save_token!(TokenKind::Panic))]
	Panic,

	#[token("assert", save_token!(TokenKind::Assert))]
	Assert,

	#[token("let", save_token!(TokenKind::Let))]
	Let,

	#[token("let!", save_token!(TokenKind::LetMut))]
	LetMut,

	#[token("if", save_token!(TokenKind::If))]
	If,

	#[token("else", save_token!(TokenKind::Else))]
	Else,

	#[token("iter", save_token!(TokenKind::Iter))]
	Iter,

	#[token("of", save_token!(TokenKind::Of))]
	Of,

	#[token("return", save_token!(TokenKind::Return))]
	Return,

	#[token("break", save_token!(TokenKind::Break))]
	Break,

	#[token("continue", save_token!(TokenKind::Continue))]
	Continue,

	#[token("import", save_token!(TokenKind::Import))]
	Import,

	#[token("export", save_token!(TokenKind::Export))]
	Export,

	// extra
	#[regex(r"\n", asi)]
	NewLine,

	// regexes
	#[regex(r"//[^\n]*", save_token!(TokenKind::LineComment))]
	LineComment,

	#[regex(r"/\*([^*]|\*[^/])*\*/", save_token!(TokenKind::BlockCommnet))]
	BlockCommnet,

	#[regex("-?(0|[1-9][0-9]*)", |lex| {
		lex.extras = Some(TokenKind::Integer);
		lex.slice().parse().ok()
	})]
	Integer(i32),

	#[regex(r#"'(?:[^']|\\'|\\n|\\t)*'"#, lex_string)]
	#[regex(r#""(?:[^"]|\\"|\\n|\\t)*""#, lex_string)]
	String(String),

	#[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| {
		lex.extras = Some(TokenKind::Identifier);
		lex.slice().to_owned()
	})]
	#[regex("[0-9]+[_a-zA-Z]+", |_| Err(LexError::InvalidIndentifier))]
	Identifier(String),
}
