use winnow::{combinator::alt, Located, PResult, Parser};

use crate::Token;

type I<'a> = Located<&'a str>;

pub fn token<'a>(i: &mut I<'a>) -> PResult<Token> {
	alt(("use".value(Token::Use),)).parse_next(i)
}
