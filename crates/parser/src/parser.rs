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
	use crate::{
		ast::{BinExpr, Ident},
		n,
	};

	#[test]
	fn parse_math_expr() {
		let res = parse("1 + 2 * 3");

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Add(
				n!(1),
				Box::new(Expr::BinExpr(BinExpr::Mul(
					n!(2),
					n!(3),
				))),
			)))
		)
	}

	#[test]
	fn parse_declare_expr() {
		let res = parse("let xy = 1 + 2");

		assert_eq!(
			res,
			Ok(Expr::Declare(
				Ident("xy".to_owned()),
				Box::new(Expr::BinExpr(BinExpr::Add(
					n!(1),
					n!(2)
				)))
			))
		)
	}
}
