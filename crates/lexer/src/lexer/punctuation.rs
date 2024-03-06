use winnow::{combinator::alt, PResult, Parser};

use super::Stream;
use crate::Token;

pub fn punctuation(i: &mut Stream<'_>) -> PResult<Token> {
	alt([
		"}".value(Token::RightBrace),
		"||".value(Token::PipePipe),
		"|".value(Token::Pipe),
		"{".value(Token::LeftBrace),
		"]".value(Token::RightBracket),
		"\n".value(Token::NewLine),
		"[".value(Token::LeftBracket),
		"@".value(Token::At),
		">=".value(Token::RightAngledBracketEqual),
		">".value(Token::RightAngledBracketEqual),
		"==".value(Token::EqualEqual),
		"=".value(Token::Equal),
		"<=".value(Token::LeftAngledBracketEqual),
		"<".value(Token::LeftAngledBracketEqual),
		":".value(Token::Colon),
		"/=".value(Token::SlashEqual),
		"/".value(Token::Slash),
		"..".value(Token::DotDot),
		".".value(Token::Dot),
		"->".value(Token::Arrow),
		"-=".value(Token::MinusEqual),
		"-".value(Token::Minus),
		",".value(Token::Comma),
		"+=".value(Token::PlusEqual),
		"+".value(Token::Plus),
		"*=".value(Token::AsteriskEqual),
		"*".value(Token::Asterisk),
		")".value(Token::RightParenthesis),
		"(".value(Token::LeftParenthesis),
		"&&".value(Token::AndAnd),
		"%=".value(Token::PercentEqual),
		"%".value(Token::Percent),
		"!=".value(Token::BangEqual),
		"!".value(Token::Bang),
	])
	.parse_next(i)
}
