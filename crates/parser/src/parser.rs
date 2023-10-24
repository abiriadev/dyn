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
		ast::{BinExpr, Boolean, Code, Ident, Literal},
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

	#[test]
	fn parse_unary_minus() {
		let res = parse(r#"- 123"#);

		assert_eq!(res, Ok(Expr::UnaryMinus(n!(123))));
	}

	#[test]
	fn does_not_support_decrement_operator() {
		let res = parse(r#"--a"#);

		assert_eq!(
			res,
			Ok(Expr::unary_minus_box(Expr::UnaryMinus(
				ident!(a)
			)))
		);
	}

	#[test]
	fn unary_minus_followed_by_multiplication() {
		let res = parse(r#"- a*b"#);

		assert_eq!(
			res,
			Ok(Expr::UnaryMinus(ident!(a)) * *ident!(b))
		);
	}

	#[test]
	fn parse_unary_not() {
		let res = parse(r#"!true"#);

		assert_eq!(
			res,
			Ok(Expr::unary_not_box(Expr::Literal(
				Literal::Boolean(Boolean(true))
			)))
		);
	}

	#[test]
	fn parse_nested_unary_not() {
		let res = parse(r#"!!a"#);

		assert_eq!(
			res,
			Ok(Expr::unary_not_box(Expr::UnaryNot(
				ident!(a)
			)))
		);
	}

	#[test]
	fn parse_and() {
		let res = parse(r#"true && false"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::and_box(
				Expr::Literal(Literal::Boolean(Boolean(true))),
				Expr::Literal(Literal::Boolean(Boolean(false)))
			)))
		);
	}

	#[test]
	fn parse_or() {
		let res = parse(r#"true || false"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::or_box(
				Expr::Literal(Literal::Boolean(Boolean(true))),
				Expr::Literal(Literal::Boolean(Boolean(false)))
			)))
		);
	}

	#[test]
	fn boolean_operator_precedence() {
		let res = parse(r#"true || false && true"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::or_box(
				Expr::Literal(Literal::Boolean(Boolean(true))),
				Expr::BinExpr(BinExpr::and_box(
					Expr::Literal(Literal::Boolean(Boolean(false))),
					Expr::Literal(Literal::Boolean(Boolean(true)))
				))
			)))
		);
	}

	#[test]
	fn parse_div() {
		let res = parse(r#"a / 123"#);

		assert_eq!(res, Ok(*ident!(a) / *n!(123)))
	}

	#[test]
	fn parse_mod() {
		let res = parse(r#"a % 123"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Mod(
				ident!(a),
				n!(123)
			)))
		)
	}

	#[test]
	fn parse_equal() {
		let res = parse(r#"1 == 3"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Equal(
				n!(1),
				n!(3)
			)))
		)
	}

	#[test]
	fn parse_not_equal() {
		let res = parse(r#"1 != 3"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::NotEqual(
				n!(1),
				n!(3)
			)))
		)
	}

	#[test]
	fn parse_less_than() {
		let res = parse(r#"1 < 2"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::LessThan(
				n!(1),
				n!(2)
			)))
		)
	}

	#[test]
	fn parse_less_than_equal() {
		let res = parse(r#"1 <= 2"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::LessThanEqual(
				n!(1),
				n!(2)
			)))
		)
	}

	#[test]
	fn parse_greater_than() {
		let res = parse(r#"1 > 2"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::GreaterThan(
				n!(1),
				n!(2)
			)))
		)
	}
}
