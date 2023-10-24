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
		ident, n,
	};

	#[test]
	fn parse_math_expr() {
		let res = parse("1 + 2 * 3");

		assert_eq!(res, Ok(*(n!(1) + n!(2) * n!(3))))
	}

	#[test]
	fn parse_declare_expr() {
		let res = parse("let xy = 1 + 2");

		assert_eq!(
			res,
			Ok(Expr::Declare(
				Ident("xy".to_owned()),
				n!(1) + n!(2)
			))
		)
	}

	#[test]
	fn parse_math_expr_involving_variable() {
		let res = parse("2 * pi * r");

		assert_eq!(
			res,
			Ok(*(n!(2) * ident!(pi) * ident!(r)))
		)
	}

	#[test]
	fn parse_indexing() {
		let res = parse("arr[123]");

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Index(
				ident!(arr),
				n!(123)
			)))
		)
	}

	#[test]
	fn parse_nested_indexing() {
		let res = parse("arr[1][2][3]");

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::index_box(
				Expr::BinExpr(BinExpr::index_box(
					Expr::BinExpr(BinExpr::Index(ident!(arr), n!(1))),
					*n!(2)
				)),
				*n!(3)
			)))
		)
	}
}
