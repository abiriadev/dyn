use lalrpop_util::lalrpop_mod;
use lexer::{LexError, SpannedLexer, Token};
use span::Spanned;

use crate::ast::{Code, Expr};

lalrpop_mod!(#[allow(clippy::type_complexity)] pub dynlang);

pub type ParseError = lalrpop_util::ParseError<usize, Token, Spanned<LexError>>;

pub fn parse(code: &str) -> Result<Expr, ParseError> {
	dynlang::ExprParser::new().parse(SpannedLexer::new(code))
}

pub fn parse_code(code: &str) -> Result<Code, ParseError> {
	dynlang::CodeParser::new().parse(SpannedLexer::new(code))
}

#[cfg(test)] mod tests;
