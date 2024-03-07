use std::fmt::{self, Display, Formatter};

use serde::Serialize;
use span::{HasSpan, Span};
use strum::EnumDiscriminants;

use crate::LexError;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum QuoteKind {
	Single,
	Double,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct QuotedString {
	pub content: String,
	pub quote: QuoteKind,
}

impl From<QuotedString> for String {
	fn from(value: QuotedString) -> Self { value.content }
}

#[derive(Debug, Clone, PartialEq, EnumDiscriminants, Serialize)]
#[strum_discriminants(name(TokenKind))]
pub enum Token {
	Whitespace,

	// Elementary arithmetics
	Plus,
	Minus,
	Asterisk,
	Slash,
	Percent,

	// Assignment operators
	Equal,
	PlusEqual,
	MinusEqual,
	AsteriskEqual,
	SlashEqual,
	PercentEqual,

	// Comparison operators
	EqualEqual,
	BangEqual,
	LeftAngledBracket,
	RightAngledBracket,
	LeftAngledBracketEqual,
	RightAngledBracketEqual,

	// Boolean operators
	AndAnd,
	PipePipe,

	// Parentheses
	LeftParenthesis,
	RightParenthesis,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,

	// etc
	Bang,
	Dot,
	DotDot,
	Comma,
	Colon,
	Pipe,
	At,
	Arrow,

	// token literals
	Nil,
	True,
	False,

	// keywords
	Let,
	LetMut,
	If,
	Else,
	Iter,
	Of,
	Return,
	Break,
	Continue,
	Use,
	Export,

	// extra
	NewLine,

	LineComment,
	BlockComment,

	Integer(i32),

	String(QuotedString),

	TemplateStringLeadingFragment(String),
	TemplateStringCentralFragment(String),
	TemplateStringTrailingFragment(String),

	Identifier(String),
}

impl Token {
	pub fn to_source(&self) -> String {
		match self {
			Token::Whitespace => "[WS]".to_owned(),
			Token::Plus => "+".to_owned(),
			Token::Minus => "-".to_owned(),
			Token::Asterisk => "*".to_owned(),
			Token::Slash => "/".to_owned(),
			Token::Percent => "%".to_owned(),
			Token::Equal => "=".to_owned(),
			Token::PlusEqual => "+=".to_owned(),
			Token::MinusEqual => "-=".to_owned(),
			Token::AsteriskEqual => "*=".to_owned(),
			Token::SlashEqual => "/=".to_owned(),
			Token::PercentEqual => "%=".to_owned(),
			Token::EqualEqual => "==".to_owned(),
			Token::BangEqual => "!=".to_owned(),
			Token::LeftAngledBracket => "<".to_owned(),
			Token::RightAngledBracket => ">".to_owned(),
			Token::LeftAngledBracketEqual => "<=".to_owned(),
			Token::RightAngledBracketEqual => ">=".to_owned(),
			Token::AndAnd => "&&".to_owned(),
			Token::PipePipe => "||".to_owned(),
			Token::LeftParenthesis => "(".to_owned(),
			Token::RightParenthesis => ")".to_owned(),
			Token::LeftBrace => "{".to_owned(),
			Token::RightBrace => "}".to_owned(),
			Token::LeftBracket => "[".to_owned(),
			Token::RightBracket => "]".to_owned(),
			Token::Bang => "!".to_owned(),
			Token::Dot => ".".to_owned(),
			Token::DotDot => "..".to_owned(),
			Token::Comma => ".".to_owned(),
			Token::Colon => ":".to_owned(),
			Token::Pipe => "|".to_owned(),
			Token::At => "@".to_owned(),
			Token::Arrow => "<-".to_owned(),
			Token::Nil => "nil".to_owned(),
			Token::True => "true".to_owned(),
			Token::False => "false".to_owned(),
			Token::Let => "let".to_owned(),
			Token::LetMut => "let!".to_owned(),
			Token::If => "if".to_owned(),
			Token::Else => "else".to_owned(),
			Token::Iter => "iter".to_owned(),
			Token::Of => "of".to_owned(),
			Token::Return => "return".to_owned(),
			Token::Break => "break".to_owned(),
			Token::Continue => "continue".to_owned(),
			Token::Use => "use".to_owned(),
			Token::Export => "export".to_owned(),
			Token::NewLine => "\\n".to_owned(), // NOTE: escape backslash
			Token::LineComment => unreachable!(), // NOTE: skipped token
			Token::BlockComment => unreachable!(), // NOTE: skipped token
			Token::Integer(v) => format!("{v}"),
			Token::String(QuotedString { content, quote }) => match quote {
				QuoteKind::Single => format!("'{content}'"),
				QuoteKind::Double => format!(r#""{content}""#),
			},
			Token::TemplateStringLeadingFragment(v) => format!(r#""{v}{{"#),
			Token::TemplateStringCentralFragment(v) => format!(r#"}}{v}{{"#),
			Token::TemplateStringTrailingFragment(v) => format!(r#"}}{v}""#),
			Token::Identifier(v) => v.to_string(),
		}
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_source())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
	pub token: Result<Token, LexError>,
	pub span: Span,
}

impl SpannedToken {
	pub fn new(token: Token, span: Span) -> Self {
		Self {
			token: Ok(token),
			span,
		}
	}

	pub fn new_err(error: LexError, span: Span) -> Self {
		Self {
			token: Err(error),
			span,
		}
	}
}

impl HasSpan for SpannedToken {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}
