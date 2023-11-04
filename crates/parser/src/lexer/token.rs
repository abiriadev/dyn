use logos::{Filter, Lexer, Logos, Skip};
use strum::EnumDiscriminants;

use super::LexError;
use crate::macros::save_token;

fn asi(lex: &mut Lexer<Token>) -> Filter<()> {
	let res = match lex.extras {
		Some(
			// identifiers
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

#[derive(Debug, Clone, PartialEq)]
pub enum QuotedString {
	Single(String),
	Double(String),
}

impl QuotedString {
	pub fn into_string(self) -> String {
		match self {
			QuotedString::Single(v) => v,
			QuotedString::Double(v) => v,
		}
	}
}

impl From<QuotedString> for String {
	fn from(value: QuotedString) -> Self { value.into_string() }
}

fn lex_string_single(lex: &mut Lexer<Token>) -> QuotedString {
	lex.extras = Some(TokenKind::String);
	let sl = lex.slice();
	QuotedString::Single(sl[1..sl.len() - 1].to_owned())
}

fn lex_string_double(lex: &mut Lexer<Token>) -> QuotedString {
	lex.extras = Some(TokenKind::String);
	let sl = lex.slice();
	QuotedString::Double(sl[1..sl.len() - 1].to_owned())
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

	// Parentheses
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
	#[regex(r"//[^\n]*", |_| Skip)]
	LineComment,

	#[regex(r"/\*([^*]|\*[^/])*\*/", |_| Skip)]
	BlockComment,

	#[regex("-?(0|[1-9][0-9]*)", |lex| {
		lex.extras = Some(TokenKind::Integer);
		lex.slice().parse().ok()
	})]
	Integer(i32),

	#[regex(r#"'(?:[^']|\\'|\\n|\\t)*'"#, lex_string_single)]
	#[regex(r#""(?:[^"]|\\"|\\n|\\t)*""#, lex_string_double)]
	String(QuotedString),

	#[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| {
		lex.extras = Some(TokenKind::Identifier);
		lex.slice().to_owned()
	})]
	#[regex("[0-9]+[_a-zA-Z]+", |_| Err(LexError::InvalidIdentifier))]
	Identifier(String),
}

impl Token {
	pub fn to_source(&self) -> String {
		match self {
			Token::Plus => "+".to_owned(),
			Token::Minus => "-".to_owned(),
			Token::Asterisk => "*".to_owned(),
			Token::Slash => "/".to_owned(),
			Token::Percent => "%".to_owned(),
			Token::Assign => "=".to_owned(),
			Token::PlusAssign => "+=".to_owned(),
			Token::MinusAssign => "-=".to_owned(),
			Token::AsteriskAssign => "*=".to_owned(),
			Token::SlashAssign => "/=".to_owned(),
			Token::PercentAssign => "%=".to_owned(),
			Token::Equal => "==".to_owned(),
			Token::NotEqual => "!=".to_owned(),
			Token::LeftAngledBracket => "<".to_owned(),
			Token::RightAngledBracket => ">".to_owned(),
			Token::LessThanEqual => "<=".to_owned(),
			Token::GreaterThanEqual => ">=".to_owned(),
			Token::DoubleAnd => "&&".to_owned(),
			Token::DoublePipe => "||".to_owned(),
			Token::LeftParenthesis => "(".to_owned(),
			Token::RightParenthesis => ")".to_owned(),
			Token::LeftBrace => "{".to_owned(),
			Token::RightBrace => "}".to_owned(),
			Token::LeftBracket => "[".to_owned(),
			Token::RightBracket => "]".to_owned(),
			Token::Bang => "!".to_owned(),
			Token::Dot => ".".to_owned(),
			Token::Comma => ".".to_owned(),
			Token::Pipe => "|".to_owned(),
			Token::At => "@".to_owned(),
			Token::Arrow => "<-".to_owned(),
			Token::Nil => "nil".to_owned(),
			Token::True => "true".to_owned(),
			Token::False => "false".to_owned(),
			Token::Panic => "panic".to_owned(),
			Token::Assert => "assert".to_owned(),
			Token::Let => "let".to_owned(),
			Token::LetMut => "let!".to_owned(),
			Token::If => "if".to_owned(),
			Token::Else => "else".to_owned(),
			Token::Iter => "iter".to_owned(),
			Token::Of => "of".to_owned(),
			Token::Return => "return".to_owned(),
			Token::Break => "break".to_owned(),
			Token::Continue => "continue".to_owned(),
			Token::Import => "import".to_owned(),
			Token::Export => "export".to_owned(),
			Token::NewLine => "\\n".to_owned(), // NOTE: escape backslash
			Token::LineComment => unreachable!(), // NOTE: skipped token
			Token::BlockComment => unreachable!(), // NOTE: skipped token
			Token::Integer(v) => format!("{v}"),
			Token::String(v) => match v {
				QuotedString::Single(v) => format!("'{v}'"),
				QuotedString::Double(v) => format!(r#""{v}""#),
			},
			Token::Identifier(v) => v.to_string(),
		}
	}
}
