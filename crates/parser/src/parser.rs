use dyn_lexer::{
	lexer::LexerConfig, LexError, SpannedLexer, SpannedToken, Token,
};
use dyn_span::Spanned;
use lalrpop_util::lalrpop_mod;

use crate::ast::{Code, Expr};

lalrpop_mod!(#[allow(clippy::type_complexity)] pub dynlang);

pub type ParseError = lalrpop_util::ParseError<usize, Token, Spanned<LexError>>;

fn lexer_adapter(
	spanned_token: SpannedToken,
) -> Result<(usize, Token, usize), Spanned<LexError>> {
	match spanned_token {
		SpannedToken {
			token: Ok(token),
			span,
		} => Ok((span.start(), token, span.end())),
		SpannedToken {
			token: Err(error),
			span,
		} => Err(Spanned::new(span, error)),
	}
}

pub fn parse(code: &str) -> Result<Expr, ParseError> {
	dynlang::ExprParser::new().parse(
		SpannedLexer::new(code, LexerConfig::default()).map(lexer_adapter),
	)
}

pub fn parse_code(code: &str) -> Result<Code, ParseError> {
	dynlang::CodeParser::new().parse(
		SpannedLexer::new(code, LexerConfig::default()).map(lexer_adapter),
	)
}

#[cfg(test)]
mod tests;
