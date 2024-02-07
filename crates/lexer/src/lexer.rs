use winnow::{combinator::alt, Located, PResult, Parser};

use crate::Token;

mod comment;
mod identifier;
mod keyword;

use identifier::identifier;
use keyword::keyword;

type I<'a> = Located<&'a str>;

pub fn token(i: &mut I<'_>) -> PResult<Token> {
	alt((keyword, identifier)).parse_next(i)
}
