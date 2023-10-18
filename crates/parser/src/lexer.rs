use logos::{Lexer, Logos};

fn lex_string(lex: &mut Lexer<Token>) -> String {
	let sl = lex.slice();
	sl[1..sl.len() - 1].to_owned()
}

#[derive(Debug, PartialEq, Logos)]
#[logos(skip r"[ \t]+")]
pub enum Token {
	// Elementary arithmetics
	#[token("+")]
	Plus,

	#[token("-")]
	Minus,

	#[token("*")]
	Asterisk,

	#[token("/")]
	Slash,

	#[token("%")]
	Percent,

	// Assignment operators
	#[token("=")]
	Assign,

	#[token("+=")]
	PlusAssign,

	#[token("-=")]
	MinusAssign,

	#[token("*=")]
	AsteriskAssign,

	#[token("/=")]
	SlashAssign,

	#[token("%=")]
	PercentAssign,

	// Comparison operators
	#[token("==")]
	Equal,

	#[token("!=")]
	NotEqual,

	#[token("<")]
	LeftAngledBracket,

	#[token(">")]
	RightAngledBracket,

	#[token("<=")]
	LessThanEqual,

	#[token(">=")]
	GreaterThanEqual,

	// Boolean operators
	#[token("&&")]
	DoubleAnd,

	#[token("||")]
	DoublePipe,

	// Parantheses
	#[token("(")]
	LeftParenthesis,

	#[token(")")]
	RightParenthesis,

	#[token("{")]
	LeftBrace,

	#[token("}")]
	RightBrace,

	#[token("[")]
	LeftBracket,

	#[token("]")]
	RightBracket,

	// etc
	#[token("!")]
	Bang,

	#[token(".")]
	Dot,

	#[token(",")]
	Comma,

	#[token("@")]
	At,

	#[token("->")]
	Arrow,

	// token literals
	#[token("nil")]
	Nil,

	#[token("true")]
	True,

	#[token("false")]
	False,

	// keywords
	#[token("panic")]
	Panic,

	#[token("assert")]
	Assert,

	#[token("let")]
	Let,

	#[token("let!")]
	LetMut,

	#[token("if")]
	If,

	#[token("else")]
	Else,

	#[token("iter")]
	Iter,

	#[token("of")]
	Of,

	#[token("return")]
	Return,

	#[token("break")]
	Break,

	#[token("continue")]
	Continue,

	#[token("import")]
	Import,

	#[token("export")]
	Export,

	// regexes
	#[regex(r"\n[ \t\n]*")]
	NewLine,

	#[regex(r"//[^\n]*")]
	LineComment,

	#[regex(r"/\*([^*]|\*[^/])+\*/")]
	BlockCommnet,

	#[regex("-?(0|[1-9][0-9]*)", |lex| lex.slice().parse().ok())]
	Integer(i32),

	#[regex(r#"'(?:[^']|\\'\\n\\t)*'"#, lex_string)]
	#[regex(r#""(?:[^"]|\\"\\n\\t)*""#, lex_string)]
	String(String),

	#[regex("[_a-zA-Z][_0-9a-zA-Z]+", |lex| lex.slice().to_owned())]
	Identifier(String),
}

#[cfg(test)]
mod tests {
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
	}
}
