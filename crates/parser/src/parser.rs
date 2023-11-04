use lalrpop_util::{lalrpop_mod, ParseError};
use span::Spanned;

use crate::{
	ast::{Code, Expr},
	lexer::{LexError, SpannedLexer, Token},
};

lalrpop_mod!(pub dynlang);

pub fn parse(
	code: &str,
) -> Result<Expr, ParseError<usize, Token, Spanned<LexError>>> {
	dynlang::ExprParser::new().parse(SpannedLexer::new(code))
}

pub fn parse_code(
	code: &str,
) -> Result<Code, ParseError<usize, Token, Spanned<LexError>>> {
	dynlang::CodeParser::new().parse(SpannedLexer::new(code))
}

#[cfg(test)] mod tests;
