use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{
	ast::Expr,
	lexer::{LexError, SpannedLexer, Token},
};

lalrpop_mod!(pub dynlang);

fn parse(code: &str) -> Result<Expr, ParseError<usize, Token, LexError>> {
	dynlang::ExprParser::new().parse(SpannedLexer::new(code))
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_eq;

	use super::*;
	use crate::ast::{BinExpr, Integer, Literal};

	#[test]
	fn parse_math_expr() {
		let res = parse("1 + 2 * 3");

		println!("{res:#?}");
		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Add(
				Box::new(Expr::Literal(Literal::Integer(
					Integer(1)
				))),
				Box::new(Expr::BinExpr(BinExpr::Mul(
					Box::new(Expr::Literal(Literal::Integer(
						Integer(2)
					))),
					Box::new(Expr::Literal(Literal::Integer(
						Integer(3)
					)))
				))),
			)))
		)
	}
}
