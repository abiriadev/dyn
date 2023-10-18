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

	// keywords
	#[token("nil")]
	Nil,

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
	#[regex("\n+")]
	NewLine,

	#[regex("[a-zA-Z]+")]
	Identifier,
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
