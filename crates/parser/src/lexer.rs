use logos::Logos;

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

	#[token("'")]
	SingleQuote,

	#[token("\"")]
	DoubleQuote,

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

	#[regex(r#""(?:[^"]|\\")*""#, |lex| {
		let sl = lex.slice();
		sl[1..sl.len() - 1].to_owned()
	})]
	String(String),

	#[regex("[a-zA-Z]+", |lex| lex.slice().to_owned())]
	Identifier(String),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lex_tag_tokens() {
		let spanned = Token::lexer("+")
			.spanned()
			.collect::<Vec<_>>();

		assert_eq!(spanned, [(Ok(Token::Plus), 0..1)]);
	}
}
