use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{
	ast::{Code, Expr},
	lexer::{LexError, SpannedLexer, Token},
};

lalrpop_mod!(pub dynlang);

pub fn parse(code: &str) -> Result<Expr, ParseError<usize, Token, LexError>> {
	dynlang::ExprParser::new().parse(SpannedLexer::new(code))
}

pub fn parse_code(
	code: &str,
) -> Result<Code, ParseError<usize, Token, LexError>> {
	dynlang::CodeParser::new().parse(SpannedLexer::new(code))
}

#[cfg(test)] mod tests;
