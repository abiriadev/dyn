use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
#[logos(skip r"[ \t\n]+")]
enum Token {
	#[token("+")]
	Plus,

	#[token("-")]
	Minus,

	#[token("*")]
	Asterisk,

	#[token("/")]
	Slash,

	#[token("=")]
	Equal,

	#[token("!")]
	Bang,

	#[token(",")]
	Comma,

	#[token("@")]
	At,

	#[token("'")]
	SingleQuote,

	#[token("\"")]
	DoubleQuote,

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

	#[token("+=")]
	PlusEqual,

	#[token("-=")]
	MinusEqual,

	#[token("*=")]
	AsteriskEqual,

	#[token("/=")]
	SlashEqual,

	#[token("&&")]
	DoubleAnd,

	#[token("||")]
	DoublePipe,

	#[token("->")]
	Arrow,

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

	#[regex("[a-zA-Z]+")]
	Identifier,
}
