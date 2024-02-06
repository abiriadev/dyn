use std::fmt::{self, Display, Formatter};

use logos::{Filter, Lexer, Logos, Skip};
use strum::EnumDiscriminants;

use super::LexError;

macro_rules! skip {
	() => {
		|lex: &mut Lexer<'_>| {
			lex.extras = false;
		}
	};
}

macro_rules! asi {
	() => {
		|lex: &mut Lexer<'_>| {
			lex.extras = true;
		}
	};
}

fn asi(lex: &mut Lexer<Token>) -> Filter<()> {
	let res = if lex.extras {
		Filter::Emit(())
	} else {
		Filter::Skip
	};
	lex.extras = false;
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
	lex.extras = true;
	let sl = lex.slice();
	QuotedString::Single(sl[1..sl.len() - 1].to_owned())
}

fn lex_string_double(lex: &mut Lexer<Token>) -> QuotedString {
	lex.extras = true;
	let sl = lex.slice();
	QuotedString::Double(sl[1..sl.len() - 1].to_owned())
}

fn lex_template_string(lex: &mut Lexer<Token>) -> String {
	lex.extras = false;
	let sl = lex.slice();
	sl[1..sl.len() - 1].to_owned()
}

fn lex_trailing_template_string(lex: &mut Lexer<Token>) -> String {
	lex.extras = true;
	let sl = lex.slice();
	sl[1..sl.len() - 1].to_owned()
}

#[derive(Debug, Clone, PartialEq, Logos, EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
#[logos(
	skip r"[ \t]+",
	error = LexError,
	extras = bool
)]
pub enum Token {
	// Elementary arithmetics
	#[token("+", skip!())]
	Plus,

	#[token("-", skip!())]
	Minus,

	#[token("*", skip!())]
	Asterisk,

	#[token("/", skip!())]
	Slash,

	#[token("%", skip!())]
	Percent,

	// Assignment operators
	#[token("=", skip!())]
	Assign,

	#[token("+=", skip!())]
	PlusAssign,

	#[token("-=", skip!())]
	MinusAssign,

	#[token("*=", skip!())]
	AsteriskAssign,

	#[token("/=", skip!())]
	SlashAssign,

	#[token("%=", skip!())]
	PercentAssign,

	// Comparison operators
	#[token("==", skip!())]
	Equal,

	#[token("!=", skip!())]
	NotEqual,

	#[token("<", skip!())]
	LeftAngledBracket,

	#[token(">", asi!())]
	RightAngledBracket,

	#[token("<=", skip!())]
	LessThanEqual,

	#[token(">=", skip!())]
	GreaterThanEqual,

	// Boolean operators
	#[token("&&", skip!())]
	DoubleAnd,

	#[token("||", skip!())]
	DoublePipe,

	// Parentheses
	#[token("(", skip!())]
	LeftParenthesis,

	#[token(")", asi!())]
	RightParenthesis,

	#[token("{", skip!())]
	LeftBrace,

	#[token("}", asi!())]
	RightBrace,

	#[token("[", skip!())]
	LeftBracket,

	#[token("]", asi!())]
	RightBracket,

	// etc
	#[token("!", skip!())]
	Bang,

	#[token(".", skip!())]
	Dot,

	#[token(",", skip!())]
	Comma,

	#[token(":", skip!())]
	Colon,

	#[token("|", skip!())]
	Pipe,

	#[token("@", skip!())]
	At,

	#[token("->", skip!())]
	Arrow,

	// token literals
	#[token("nil", skip!())]
	Nil,

	#[token("true", skip!())]
	True,

	#[token("false", skip!())]
	False,

	// keywords
	#[token("panic", skip!())]
	Panic,

	#[token("assert", skip!())]
	Assert,

	#[token("let", skip!())]
	Let,

	#[token("let!", skip!())]
	LetMut,

	#[token("if", skip!())]
	If,

	#[token("else", skip!())]
	Else,

	#[token("iter", skip!())]
	Iter,

	#[token("of", skip!())]
	Of,

	#[token("return", skip!())]
	Return,

	#[token("break", skip!())]
	Break,

	#[token("continue", skip!())]
	Continue,

	#[token("import", skip!())]
	Import,

	#[token("export", skip!())]
	Export,

	// extra
	#[regex(r"\n", asi)]
	NewLine,

	// regexes
	#[regex(r"//[^\n]*", |_| Skip)]
	LineComment,

	#[regex(r"/\*([^*]|\*[^/])*\*/")]
	BlockComment,

	#[regex("-?(0|[1-9][0-9]*)", |lex| {
		lex.extras = true;
		lex.slice().parse().ok()
	})]
	Integer(i32),

	#[regex(r#"'(?:[^']|\\'|\\n|\\t)*'"#, lex_string_single)]
	#[regex(r#""(?:[^"]|\\"|\\n|\\t)*""#, lex_string_double)]
	String(QuotedString),

	#[regex(r#""(?:[^']|\\'|\\n|\\t)*\{"#, lex_template_string)]
	TemplateStringLeadingFragment(String),

	#[regex(r#"\}(?:[^']|\\'|\\n|\\t)*\{"#, lex_template_string)]
	TemplateStringCentralFragment(String),

	#[regex(r#"\}(?:[^']|\\'|\\n|\\t)*'"#, lex_trailing_template_string)]
	TemplateStringTrailingFragment(String),

	#[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| {
		lex.extras = true;
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
			Token::Colon => ":".to_owned(),
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
