use std::fmt::{self, Display, Formatter};

use logos::{Filter, Lexer, Logos};
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
pub enum QuoteKind {
	Single,
	Double,
}

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
	Equal,

	#[token("+=", skip!())]
	PlusEqual,

	#[token("-=", skip!())]
	MinusEqual,

	#[token("*=", skip!())]
	AsteriskEqual,

	#[token("/=", skip!())]
	SlashEqual,

	#[token("%=", skip!())]
	PercentEqual,

	// Comparison operators
	#[token("==", skip!())]
	EqualEqual,

	#[token("!=", skip!())]
	BangEqual,

	#[token("<", skip!())]
	LeftAngledBracket,

	#[token(">", asi!())]
	RightAngledBracket,

	#[token("<=", skip!())]
	LeftAngledBracketEqual,

	#[token(">=", skip!())]
	RightAngledBracketEqual,

	// Boolean operators
	#[token("&&", skip!())]
	AndAnd,

	#[token("||", skip!())]
	PipePipe,

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

	#[token("use", skip!())]
	Use,

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

	#[regex(r#"'(?:[^'{}]|\\['nt{}])*'"#, lex_string_single)]
	#[regex(r#""(?:[^"{}]|\\["nt{}])*""#, lex_string_double)]
	String { content: String, quote: QuoteKind },

	#[regex(r#""(?:[^"{}]|\\["nt{}])*\{"#, lex_template_string)]
	TemplateStringLeadingFragment(String),

	#[regex(r#"\}(?:[^"{}]|\\["nt{}])*\{"#, lex_template_string)]
	TemplateStringCentralFragment(String),

	#[regex(r#"\}(?:[^"{}]|\\["nt{}])*""#, lex_trailing_template_string)]
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
			Token::String { content, quote } => match quote {
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
