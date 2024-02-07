use winnow::{combinator::alt, Located, PResult, Parser};

use crate::Token;

mod comment;
mod identifier;
mod keyword;

use identifier::identifier;
use keyword::keyword;

type Stream<'a> = Located<&'a str>;

pub fn token(i: &mut Stream<'_>) -> PResult<Token> {
	alt((keyword, identifier)).parse_next(i)
}
