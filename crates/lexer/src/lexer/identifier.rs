use winnow::{
	token::{one_of, take_while},
	PResult, Parser,
};

use super::I;
use crate::Token;

pub fn identifier(input: &mut I<'_>) -> PResult<Token> {
	(
		one_of(|c: char| c.is_alphabetic() || c == '_'),
		take_while(0.., |c: char| {
			c.is_alphanumeric() || c == '_'
		}),
	)
		.recognize()
		.map(|ident: &str| Token::Identifier(ident.to_owned()))
		.parse_next(input)
}
