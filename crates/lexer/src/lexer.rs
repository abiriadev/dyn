use winnow::{combinator::alt, Located, PResult, Parser};

use crate::Token;

mod identifier;

use identifier::identifier;

type I<'a> = Located<&'a str>;

pub fn token<'a>(i: &mut I<'a>) -> PResult<Token> {
	alt([
		"+".value(Token::Plus),
		"-".value(Token::Minus),
		"*".value(Token::Asterisk),
		"/".value(Token::Slash),
		"%".value(Token::Percent),
		"=".value(Token::Equal),
		"+=".value(Token::PlusEqual),
		"-=".value(Token::MinusEqual),
		"*=".value(Token::AsteriskEqual),
		"/=".value(Token::SlashEqual),
		"%=".value(Token::PercentEqual),
		"==".value(Token::EqualEqual),
		"!=".value(Token::BangEqual),
		"<".value(Token::LeftAngledBracketEqual),
		">".value(Token::RightAngledBracketEqual),
		"<=".value(Token::LeftAngledBracketEqual),
		">=".value(Token::RightAngledBracketEqual),
		"&&".value(Token::AndAnd),
		"||".value(Token::PipePipe),
		"(".value(Token::LeftParenthesis),
		")".value(Token::RightParenthesis),
		"{".value(Token::LeftBrace),
		"}".value(Token::RightBrace),
		"[".value(Token::LeftBracket),
		"]".value(Token::RightBracket),
		"!".value(Token::Bang),
		".".value(Token::Dot),
		",".value(Token::Comma),
		":".value(Token::Colon),
		"|".value(Token::Pipe),
		"@".value(Token::At),
		"->".value(Token::Arrow),
		"nil".value(Token::Nil),
		"true".value(Token::True),
		"false".value(Token::False),
		"let".value(Token::Let),
		"let!".value(Token::LetMut),
		"if".value(Token::If),
		"else".value(Token::Else),
		"iter".value(Token::Iter),
		"of".value(Token::Of),
		"return".value(Token::Return),
		"break".value(Token::Break),
		"continue".value(Token::Continue),
		"use".value(Token::Use),
		"export".value(Token::Export),
		r"\n".value(Token::NewLine),
	])
	.parse_next(i)
}
