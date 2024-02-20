use winnow::{combinator::alt, PResult, Parser};

use super::Stream;
use crate::Token;

pub fn punctuation(i: &mut Stream<'_>) -> PResult<Token> {
	alt([
		"+=".value(Token::PlusEqual),
		"-=".value(Token::MinusEqual),
		"*=".value(Token::AsteriskEqual),
		"/=".value(Token::SlashEqual),
		"%=".value(Token::PercentEqual),
		"+".value(Token::Plus),
		"-".value(Token::Minus),
		"*".value(Token::Asterisk),
		"/".value(Token::Slash),
		"%".value(Token::Percent),
		"==".value(Token::EqualEqual),
		"!=".value(Token::BangEqual),
		"=".value(Token::Equal),
		"<=".value(Token::LeftAngledBracketEqual),
		">=".value(Token::RightAngledBracketEqual),
		"<".value(Token::LeftAngledBracketEqual),
		">".value(Token::RightAngledBracketEqual),
		"&&".value(Token::AndAnd),
		"||".value(Token::PipePipe),
		"(".value(Token::LeftParenthesis),
		")".value(Token::RightParenthesis),
		"{".value(Token::LeftBrace),
		"}".value(Token::RightBrace),
		"[".value(Token::LeftBracket),
		"]".value(Token::RightBracket),
		"!".value(Token::Bang),
		"..".value(Token::DotDot),
		".".value(Token::Dot),
		",".value(Token::Comma),
		":".value(Token::Colon),
		"|".value(Token::Pipe),
		"@".value(Token::At),
		"->".value(Token::Arrow),
		"\n".value(Token::NewLine),
	])
	.parse_next(i)
}
