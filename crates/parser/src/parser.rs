use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{
	ast::Expr,
	lexer::{LexError, SpannedLexer, Token},
};

lalrpop_mod!(pub dynlang);

pub fn parse(code: &str) -> Result<Expr, ParseError<usize, Token, LexError>> {
	dynlang::ExprParser::new().parse(SpannedLexer::new(code))
}

#[cfg(test)]
mod tests {
	use indoc::indoc;
	use pretty_assertions::assert_eq;

	use super::*;
	use crate::{
		arr,
		ast::{BinExpr, Ident},
		code, fal, ident, n, str, tru,
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
				code![*ident!(x)]
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
				code![]
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
				code![*n!(1), *n!(2)]
			)))
		)
	}

	#[test]
	fn parse_function_call_spanning_multiple_lines() {
		let res = parse(indoc! {r#"
			add(
				1,
				2,
			)"#});

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(add),
				code![*n!(1), *n!(2)]
			)))
		)
	}

	#[test]
	fn parse_function_call_spanning_multiple_lines_without_trailing_comma() {
		let res = parse(indoc! {r#"
			add(
				1,
				2
			)"#});

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(add),
				code![*n!(1), *n!(2)]
			)))
		)
	}

	#[test]
	fn parse_function_call_spanning_multiple_lines_separated_by_only_newlines()
	{
		let res = parse(indoc! {r#"
			add(
				1
				2
			)"#});

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Call(
				ident!(add),
				code![*n!(1), *n!(2)]
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
				code![*ident!(a), *ident!(b)]
			)))
		)
	}

	#[test]
	fn nested_call() {
		let res = parse(r#"(a())(b())"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::call_box(
				Expr::BinExpr(BinExpr::Call(ident!(a), code![])),
				code![Expr::BinExpr(BinExpr::Call(ident!(b), code![]))]
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
					Expr::BinExpr(BinExpr::call_box(*ident!(f), code![
						*ident!(x)
					],)),
					code![*ident!(y)],
				)),
				code![*ident!(z)],
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

		assert_eq!(res, Ok(Expr::UnaryNot(tru!())));
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
			Ok(Expr::BinExpr(BinExpr::And(
				tru!(),
				fal!()
			)))
		);
	}

	#[test]
	fn parse_or() {
		let res = parse(r#"true || false"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::Or(
				tru!(),
				fal!()
			)))
		);
	}

	#[test]
	fn boolean_operator_precedence() {
		let res = parse(r#"true || false && true"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(BinExpr::or_box(
				*tru!(),
				Expr::BinExpr(BinExpr::And(fal!(), tru!()))
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

		assert_eq!(res, Ok(*ident!(a) % *n!(123)))
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

	#[test]
	fn parse_greater_than_equal() {
		let res = parse(r#"1 >= 2"#);

		assert_eq!(
			res,
			Ok(Expr::BinExpr(
				BinExpr::GreaterThanEqual(n!(1), n!(2))
			))
		)
	}

	#[test]
	fn parse_nested_declare() {
		let res = parse(r#"let a = let b = 123"#);

		assert_eq!(
			res,
			Ok(Expr::declare_box(
				Ident("a".to_owned()),
				Expr::Declare(Ident("b".to_owned()), n!(123))
			))
		)
	}

	#[test]
	fn parse_mutable_declare() {
		let res = parse(r#"let! v = []"#);

		assert_eq!(
			res,
			Ok(Expr::declare_mut_box(
				Ident("v".to_owned()),
				*arr![]
			))
		)
	}

	#[test]
	fn parse_assign() {
		let res = parse(r#"a = 123"#);

		assert_eq!(
			res,
			Ok(Expr::assign_box(
				Ident("a".to_owned()),
				*n!(123)
			))
		)
	}

	#[test]
	fn parse_nested_assign() {
		let res = parse(r#"a = b = c"#);

		assert_eq!(
			res,
			Ok(Expr::assign_box(
				Ident("a".to_owned()),
				Expr::assign_box(Ident("b".to_owned()), *ident!(c),)
			))
		)
	}

	#[test]
	fn parse_nested_assign2() {
		let res = parse(r#"a = !(b = c + 2)"#);

		assert_eq!(
			res,
			Ok(Expr::assign_box(
				Ident("a".to_owned()),
				Expr::unary_not_box(Expr::assign_box(
					Ident("b".to_owned()),
					*ident!(c) + *n!(2)
				))
			))
		)
	}

	#[test]
	fn parse_mixed_assignments() {
		let res = parse(r#"a = let b = 123 + (let! c = 1)"#);

		assert_eq!(
			res,
			Ok(Expr::assign_box(
				Ident("a".to_owned()),
				Expr::declare_box(
					Ident("b".to_owned()),
					*n!(123) + Expr::DeclareMut(Ident("c".to_owned()), n!(1))
				)
			))
		)
	}

	#[test]
	fn parse_if_expr() {
		let res = parse(r#"if true { print("it's true!") }"#);

		assert_eq!(
			res,
			Ok(Expr::If {
				condition: tru!(),
				yes: code![Expr::BinExpr(BinExpr::Call(ident!(print), code![
					*str!("it's true!")
				]))]
			})
		)
	}

	#[test]
	fn parse_if_expr2() {
		let res = parse(indoc! {r#"if a - 4 >= 0 {abc()
		def()}"#});

		assert_eq!(
			res,
			Ok(Expr::If {
				condition: Box::new(Expr::BinExpr(
					BinExpr::GreaterThanEqual(ident!(a) - n!(4), n!(0))
				)),
				yes: code![
					Expr::BinExpr(BinExpr::Call(ident!(abc), code![])),
					Expr::BinExpr(BinExpr::Call(ident!(def), code![]))
				]
			})
		)
	}

	#[test]
	fn parse_nested_if() {
		let res = parse(r#"if true { if false {} }"#);

		assert_eq!(
			res,
			Ok(Expr::If {
				condition: tru!(),
				yes: code![Expr::If {
					condition: fal!(),
					yes: code![]
				}]
			})
		)
	}

	#[test]
	fn parse_else_expression() {
		let res = parse(r#"if a > b { fetch() } else { cancel() }"#);

		assert_eq!(
			res,
			Ok(Expr::IfElse {
				condition: Box::new(Expr::BinExpr(BinExpr::GreaterThan(
					ident!(a),
					ident!(b)
				))),
				yes: code![Expr::BinExpr(BinExpr::Call(
					ident!(fetch),
					code![]
				))],
				no: code![Expr::BinExpr(BinExpr::Call(
					ident!(cancel),
					code![]
				))]
			})
		)
	}

	#[test]
	#[ignore]
	fn parse_expr_across_multiple_lines() {
		let res = parse(indoc! {r#"
			1
			+
			2
		"#});

		// TODO: fix test
		assert_eq!(res, Ok(*n!(1) + *n!(2)))
	}

	#[test]
	fn parse_return_expr() {
		let res = parse("return res");

		assert_eq!(res, Ok(Expr::Return(ident!(res))));
	}

	#[test]
	fn parse_nested_return_expr() {
		let res = parse("return return res");

		assert_eq!(
			res,
			Ok(Expr::return_box(Expr::Return(ident!(
				res
			))))
		);
	}

	#[test]
	fn parse_break_expr() {
		let res = parse("break res");

		assert_eq!(res, Ok(Expr::Break(ident!(res))));
	}

	#[test]
	fn parse_nested_break_expr() {
		let res = parse("break break res");

		assert_eq!(
			res,
			Ok(Expr::break_box(Expr::Break(ident!(
				res
			))))
		);
	}

	#[test]
	fn parse_continue_expr() {
		let res = parse("continue res");

		assert_eq!(res, Ok(Expr::Continue(ident!(res))));
	}

	#[test]
	fn parse_nested_continue_expr() {
		let res = parse("continue continue res");

		assert_eq!(
			res,
			Ok(Expr::continue_box(Expr::Continue(
				ident!(res)
			)))
		);
	}

	#[test]
	fn parse_panic_expr() {
		let res = parse("panic res");

		assert_eq!(res, Ok(Expr::Panic(ident!(res))));
	}

	#[test]
	fn parse_nested_panic_expr() {
		let res = parse("panic panic res");

		assert_eq!(
			res,
			Ok(Expr::panic_box(Expr::Panic(ident!(
				res
			))))
		);
	}

	#[test]
	fn parse_assert_expr() {
		let res = parse("assert res");

		assert_eq!(res, Ok(Expr::Assert(ident!(res))));
	}

	#[test]
	fn parse_nested_assert_expr() {
		let res = parse("assert assert res");

		assert_eq!(
			res,
			Ok(Expr::assert_box(Expr::Assert(ident!(
				res
			))))
		);
	}

	#[test]
	fn parse_add_assign() {
		let res = parse("a += 1");

		assert_eq!(
			res,
			Ok(Expr::AddAssign(
				Ident("a".to_owned()),
				n!(1)
			))
		);
	}

	#[test]
	fn parse_sub_assign() {
		let res = parse("a -= 1");

		assert_eq!(
			res,
			Ok(Expr::SubAssign(
				Ident("a".to_owned()),
				n!(1)
			))
		);
	}
}
