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
		arr,
		ast::{Array, BinExpr, Code, Ident},
		ident, n, str,
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

	#[test]
	fn parse_indexing_expr() {
		let res = parse("(a + b)[123]");

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Index(
				ident!(a) + ident!(b),
				n!(123)
			)))
		)
	}

	#[test]
	fn parse_array() {
		let res = parse(r#"[1, a, "str"]"#);

		assert_eq!(
			res,
			Ok(*arr![*n!(1), *ident!(a), *str!("str")])
		)
	}

	#[test]
	fn parse_empty_array() {
		let res = parse(r#"[]"#);

		assert_eq!(res, Ok(*arr![]))
	}

	#[test]
	fn parse_arrary_with_one_element() {
		let res = parse(r#"[1]"#);

		assert_eq!(res, Ok(*arr![*n!(1)]))
	}

	#[test]
	fn parse_nested_array() {
		let res = parse(r#"[[[]]]"#);

		assert_eq!(res, Ok(*arr![*arr![*arr![]]]))
	}

	#[test]
	fn parse_nested_array2() {
		let res = parse(r#"[[[], []], [[], []]]"#);

		assert_eq!(
			res,
			Ok(*arr![*arr![*arr![], *arr![]], *arr![
				*arr![],
				*arr![]
			]])
		)
	}

	#[test]
	fn parse_array_with_indexing() {
		let res = parse(r#"[[]][[]]"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Index(
				arr![*arr![]],
				arr![]
			)))
		)
	}

	#[test]
	fn parse_function_call() {
		let res = parse(r#"func(x)"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(func),
				Code(vec![*ident!(x)])
			)))
		)
	}

	#[test]
	fn parse_function_call_without_arguments() {
		let res = parse(r#"func()"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(func),
				Code(vec![])
			)))
		)
	}

	#[test]
	fn parse_function_call_with_more_than_one_argument() {
		let res = parse(r#"add(1, 2)"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(add),
				Code(vec![*n!(1), *n!(2)])
			)))
		)
	}

	#[test]
	fn call_expression() {
		let res = parse(r#"(c + d)(a, b)"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(c) + ident!(d),
				Code(vec![*ident!(a), *ident!(b)])
			)))
		)
	}

	#[test]
	fn nested_call() {
		let res = parse(r#"(a())(b())"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::call_box(
				Expr::BinExpr(BinExpr::Call(ident!(a), Code(vec![]))),
				Code(vec![Expr::BinExpr(BinExpr::Call(
					ident!(b),
					Code(vec![])
				))])
			)))
		)
	}

	#[test]
	fn nested_call2() {
		let res = parse(r#"f(x)(y)(z)"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::call_box(
				Expr::BinExpr(BinExpr::call_box(
					Expr::BinExpr(BinExpr::call_box(
						*ident!(f),
						Code(vec![*ident!(x)]),
					)),
					Code(vec![*ident!(y)]),
				)),
				Code(vec![*ident!(z)]),
			)))
		)
	}

	#[test]
	fn parse_sub() {
		let res = parse(r#"a - 123"#);

		assert_eq!(res, Ok(*ident!(a) - *n!(123)))
	}

	#[test]
	fn parse_sequencial_sub() {
		let res = parse(r#"a - b - c - d"#);

		assert_eq!(
			res,
			Ok(*(((ident!(a) - ident!(b)) - ident!(c)) - ident!(d)))
		)
	}
}
