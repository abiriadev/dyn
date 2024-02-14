use winnow::{
	token::{one_of, take_while},
	PResult, Parser,
};

use super::Stream;
use crate::Token;

pub fn identifier(input: &mut Stream<'_>) -> PResult<Token> {
	(
		one_of(|c: char| c.is_alphabetic() || c == '_'),
		take_while(0.., |c: char| {
			c.is_alphanumeric() || c == '_'
		}),
	)
		.recognize()
		.map(|ident: &str| match ident {
			"nil" => Token::Nil,
			"true" => Token::True,
			"false" => Token::False,
			"let" => Token::Let,
			"if" => Token::If,
			"else" => Token::Else,
			"iter" => Token::Iter,
			"of" => Token::Of,
			"return" => Token::Return,
			"break" => Token::Break,
			"continue" => Token::Continue,
			"use" => Token::Use,
			"export" => Token::Export,
			_ => Token::Identifier(ident.to_owned()),
		})
		.parse_next(input)
}
