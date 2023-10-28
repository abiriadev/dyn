use indoc::indoc;
use pretty_assertions::assert_eq;

use super::*;
use crate::{
	ast::{BinExpr, BinExprKind, ExprKind, Function, Parameters},
	macros::{arr, call, call_ident, code, fal, ident, n, nil, str, tru, var},
};

#[test]
fn parse_math_expr() {
	let res = parse("1 + 2 * 3");

	assert_eq!(res, Ok(*(n!(1 0) + n!(2 4) * n!(3 8))))
}

#[test]
fn parse_declare_expr() {
	let res = parse("let xy = 1 + 2");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Declare(var!(xy 4..6), n!(1 9) + n!(2 13)),
			0..0
		))
	)
}

#[test]
fn parse_math_expr_involving_variable() {
	let res = parse("2 * pi * r");

	assert_eq!(
		res,
		Ok(*(n!(2 0) * ident!(pi 4..6) * ident!(r 9..10)))
	)
}

#[test]
fn parse_indexing() {
	let res = parse("arr[123]");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Index(ident!(arr 0..3), n!(123 4)),
			0..0
		))
	)
}

#[test]
fn parse_nested_indexing() {
	let res = parse("arr[1][2][3]");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::index_box(
				Expr::new(
					ExprKind::index_box(
						Expr::new(
							ExprKind::Index(ident!(arr 0..3), n!(1 4)),
							0..0
						),
						*n!(2 7)
					),
					0..0
				),
				*n!(3 10)
			),
			0..0
		))
	)
}

#[test]
fn parse_indexing_expr() {
	let res = parse("(a + b)[123]");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Index(
				ident!(a 1..2) + ident!(b 5..6),
				n!(123 8)
			),
			0..0
		))
	)
}

#[test]
fn parse_array() {
	let res = parse(r#"[1, a, "str"]"#);

	assert_eq!(
		res,
		Ok(*arr![
			*n!(1 1),
			*ident!(a 4..5),
			*str!("str" 7);
			0..13
		])
	)
}

#[test]
fn parse_empty_array() {
	let res = parse(r#"[]"#);

	assert_eq!(res, Ok(*arr![;0]))
}

#[test]
fn parse_arrary_with_one_element() {
	let res = parse(r#"[1]"#);

	assert_eq!(res, Ok(*arr![*n!(1 1); 0..3]))
}

#[test]
fn parse_nested_array() {
	let res = parse(r#"[[[]]]"#);

	assert_eq!(
		res,
		Ok(*arr![*arr![*arr![;2]; 1..5]; 0..6])
	)
}

#[test]
fn parse_nested_array2() {
	let res = parse(r#"[[[], []], [[], []]]"#);

	assert_eq!(
		res,
		Ok(*arr![
			*arr![
				*arr![;2],
				*arr![;6];
				1..9
			],
			*arr![
				*arr![;12],
				*arr![; 16];
				11..19
			];
			0..20
		])
	)
}

#[test]
fn parse_array_with_indexing() {
	let res = parse(r#"[[]][[]]"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Index(arr![*arr![;1]; 0..4], arr![;5]),
			0..0
		))
	)
}

#[test]
fn parse_function_call() {
	let res = parse(r#"func(x)"#);

	assert_eq!(
		res,
		Ok(call_ident!(func 0..4 (*ident!(x 5..6))))
	)
}

#[test]
fn parse_function_call_without_arguments() {
	let res = parse(r#"func()"#);

	assert_eq!(res, Ok(call_ident!(func 0..4 ())))
}

#[test]
fn parse_function_call_with_more_than_one_argument() {
	let res = parse(r#"add(1, 2)"#);

	assert_eq!(
		res,
		Ok(call_ident!(add 0..3 (*n!(1 4), *n!(2 7))))
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
		Ok(call_ident!(add 0..3 (*n!(1 6), *n!(2 10))))
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
		Ok(call_ident!(add 0..3 (*n!(1 6), *n!(2 10))))
	)
}

#[test]
fn parse_function_call_spanning_multiple_lines_separated_by_only_newlines() {
	let res = parse(indoc! {r#"
			add(
				1
				2
			)"#});

	assert_eq!(
		res,
		Ok(call_ident!(add 0..3 (*n!(1 6), *n!(2 9))))
	)
}

#[test]
fn call_expression() {
	let res = parse(r#"(c + d)(a, b)"#);

	assert_eq!(
		res,
		Ok(
			*call!(ident!(c 1..2) + ident!(d 5..6); (*ident!(a 8..9), *ident!(b 11..12)))
		)
	)
}

#[test]
fn nested_call() {
	let res = parse(r#"(a())(b())"#);

	assert_eq!(
		res,
		Ok(*call!(Box::new(call_ident!(a 1..2 ())); (call_ident!(b 6..7 ()))))
	)
}

#[test]
fn nested_call2() {
	let res = parse(r#"f(x)(y)(z)"#);

	assert_eq!(
		res,
		Ok(*call!(
			call!(
				Box::new(call_ident!(f 0..1 (*ident!(x 2..3))));
				(*ident!(y 5..6)));
			(*ident!(z 8..9))
		)),
	)
}

#[test]
fn parse_sub() {
	let res = parse(r#"a - 123"#);

	assert_eq!(res, Ok(*ident!(a 0..1) - *n!(123 4)))
}

#[test]
fn parse_sequencial_sub() {
	let res = parse(r#"a - b - c - d"#);

	assert_eq!(
		res,
		Ok(
			*(((ident!(a 0..1) - ident!(b 4..5)) - ident!(c 8..9))
				- ident!(d 12..13))
		)
	)
}

#[test]
fn parse_unary_minus() {
	let res = parse(r#"- 123"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::UnaryMinus(n!(123 2)),
			0..0
		))
	);
}

#[test]
fn does_not_support_decrement_operator() {
	let res = parse(r#"--a"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::unary_minus_box(Expr::new(
				ExprKind::UnaryMinus(ident!(a 2..3)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn unary_minus_followed_by_multiplication() {
	let res = parse(r#"- a*b"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::UnaryMinus(ident!(a 2..3)),
			0..0
		) * *ident!(b 4..5))
	);
}

#[test]
fn parse_unary_not() {
	let res = parse(r#"!true"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::UnaryNot(tru!(1)),
			0..0
		))
	);
}

#[test]
fn parse_nested_unary_not() {
	let res = parse(r#"!!a"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::unary_not_box(Expr::new(
				ExprKind::UnaryNot(ident!(a 2..3)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn parse_and() {
	let res = parse(r#"true && false"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::And,
				lhs: tru!(0),
				rhs: fal!(8)
			}),
			0..0
		))
	);
}

#[test]
fn parse_or() {
	let res = parse(r#"true || false"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::Or,
				lhs: tru!(0),
				rhs: fal!(8)
			}),
			0..0
		))
	);
}

#[test]
fn boolean_operator_precedence() {
	let res = parse(r#"true || false && true"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::Or,
				lhs: tru!(0),
				rhs: Box::new(Expr::new(
					ExprKind::BinExpr(BinExpr {
						span: (0..0).into(),
						op: BinExprKind::And,
						lhs: fal!(8),
						rhs: tru!(17)
					}),
					0..0
				))
			}),
			0..0
		))
	);
}

#[test]
fn parse_div() {
	let res = parse(r#"a / 123"#);

	assert_eq!(res, Ok(*ident!(a 0..1) / *n!(123 4)))
}

#[test]
fn parse_mod() {
	let res = parse(r#"a % 123"#);

	assert_eq!(res, Ok(*ident!(a 0..1) % *n!(123 4)))
}

#[test]
fn parse_equal() {
	let res = parse(r#"1 == 3"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::Equal,
				lhs: n!(1 0),
				rhs: n!(3 5)
			}),
			0..0
		))
	)
}

#[test]
fn parse_not_equal() {
	let res = parse(r#"1 != 3"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::NotEqual,
				lhs: n!(1 0),
				rhs: n!(3 5)
			}),
			0..0
		))
	)
}

#[test]
fn parse_less_than() {
	let res = parse(r#"1 < 2"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::LessThan,
				lhs: n!(1 0),
				rhs: n!(2 4)
			}),
			0..0
		))
	)
}

#[test]
fn parse_less_than_equal() {
	let res = parse(r#"1 <= 2"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::LessThanEqual,
				lhs: n!(1 0),
				rhs: n!(2 5)
			}),
			0..0
		))
	)
}

#[test]
fn parse_greater_than() {
	let res = parse(r#"1 > 2"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::GreaterThan,
				lhs: n!(1 0),
				rhs: n!(2 4)
			}),
			0..0
		))
	)
}

#[test]
fn parse_greater_than_equal() {
	let res = parse(r#"1 >= 2"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::BinExpr(BinExpr {
				span: (0..0).into(),
				op: BinExprKind::GreaterThanEqual,
				lhs: n!(1 0),
				rhs: n!(2 5)
			}),
			0..0
		))
	)
}

#[test]
fn parse_nested_declare() {
	let res = parse(r#"let a = let b = 123"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::declare_box(
				var!(a 4..5),
				Expr::new(
					ExprKind::Declare(var!(b 12..13), n!(123 16)),
					0..0
				)
			),
			0..0
		),)
	)
}

#[test]
fn parse_mutable_declare() {
	let res = parse(r#"let! v = []"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::declare_mut_box(var!(v 5..6), *arr![;9]),
			0..0
		))
	)
}

#[test]
fn parse_assign() {
	let res = parse(r#"a = 123"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::assign_box(var!(a 0..1), *n!(123 4)),
			0..0
		))
	)
}

#[test]
fn parse_nested_assign() {
	let res = parse(r#"a = b = c"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::assign_box(
				var!(a 0..1),
				Expr::new(
					ExprKind::assign_box(var!(b 4..5), *ident!(c 8..9)),
					0..0
				)
			),
			0..0
		),)
	)
}

#[test]
fn parse_nested_assign2() {
	let res = parse(r#"a = !(b = c + 2)"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::assign_box(
				var!(a 0..1),
				Expr::new(
					ExprKind::unary_not_box(Expr::new(
						ExprKind::assign_box(
							var!(b 6..7),
							*ident!(c 10..11) + *n!(2 14)
						),
						0..0
					)),
					0..0
				)
			),
			0..0
		))
	)
}

#[test]
fn parse_mixed_assignments() {
	let res = parse(r#"a = let b = 123 + (let! c = 1)"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::assign_box(
				var!(a 0..1),
				Expr::new(
					ExprKind::declare_box(
						var!(b 8..9),
						*n!(123 12)
							+ Expr::new(
								ExprKind::DeclareMut(var!(c 24..25), n!(1 28)),
								0..0
							)
					),
					0..0
				)
			),
			0..0
		))
	)
}

#[test]
fn parse_if_expr() {
	let res = parse(r#"if true { print("it's true!") }"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::If {
				condition: tru!(3),
				yes: code! {
				call_ident!(print 10..15 (*str!("it's true!" 16)))
				}
			},
			0..0
		))
	)
}

#[test]
fn parse_if_expr2() {
	let res = parse(indoc! {r#"if a - 4 >= 0 {abc()
	def()}"#});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::If {
				condition: Box::new(Expr::new(
					ExprKind::BinExpr(BinExpr {
						span: (0..0).into(),
						op: BinExprKind::GreaterThanEqual,
						lhs: ident!(a 3..4) - n!(4 7),
						rhs: n!(0 12)
					}),
					0..0
				)),
				yes: code! {
				call_ident!(abc 15..18 ()),
				call_ident!(def 21..24 ())
				}
			},
			0..0
		))
	)
}

#[test]
fn parse_nested_if() {
	let res = parse(r#"if true { if false {} }"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::If {
				condition: tru!(3),
				yes: code! {
					Expr::new(ExprKind::If {
						condition: fal!(13),
						yes: code! {}
					}, 0..0)
				}
			},
			0..0
		))
	)
}

#[test]
fn parse_else_expression() {
	let res = parse(r#"if a > b { fetch() } else { cancel() }"#);

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::IfElse {
				condition: Box::new(Expr::new(
					ExprKind::BinExpr(BinExpr {
						span: (0..0).into(),
						op: BinExprKind::GreaterThan,
						lhs: ident!(a 3..4),
						rhs: ident!(b 7..8)
					}),
					0..0
				)),
				yes: code! { call_ident!(fetch 11..16 ()) },
				no: code! { call_ident!(cancel 28..34 ()) }
			},
			0..0
		))
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

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Return(ident!(res 7..10)),
			0..0
		))
	);
}

#[test]
fn parse_nested_return_expr() {
	let res = parse("return return res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::return_box(Expr::new(
				ExprKind::Return(ident!(
					res 14..17
				)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn parse_break_expr() {
	let res = parse("break res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Break(ident!(res 6..9)),
			0..0
		))
	);
}

#[test]
fn parse_nested_break_expr() {
	let res = parse("break break res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::break_box(Expr::new(
				ExprKind::Break(ident!(
					res 12..15
				)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn parse_continue_expr() {
	let res = parse("continue res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Continue(ident!(res 9..12)),
			0..0
		))
	);
}

#[test]
fn parse_nested_continue_expr() {
	let res = parse("continue continue res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::continue_box(Expr::new(
				ExprKind::Continue(ident!(
					res 18..21
				)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn parse_panic_expr() {
	let res = parse("panic res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Panic(ident!(res 6..9)),
			0..0
		))
	);
}

#[test]
fn parse_nested_panic_expr() {
	let res = parse("panic panic res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::panic_box(Expr::new(
				ExprKind::Panic(ident!(
					res 12..15
				)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn parse_assert_expr() {
	let res = parse("assert res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Assert(ident!(res 7..10)),
			0..0
		))
	);
}

#[test]
fn parse_nested_assert_expr() {
	let res = parse("assert assert res");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::assert_box(Expr::new(
				ExprKind::Assert(ident!(
					res 14..17
				)),
				0..0
			)),
			0..0
		))
	);
}

#[test]
fn parse_add_assign() {
	let res = parse("a += 1");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::AddAssign(var!(a 0..1), n!(1 5)),
			0..0
		))
	);
}

#[test]
fn parse_sub_assign() {
	let res = parse("a -= 1");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::SubAssign(var!(a 0..1), n!(1 5)),
			0..0
		))
	);
}

#[test]
fn parse_mul_assign() {
	let res = parse("a *= 1");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::MulAssign(var!(a 0..1), n!(1 5)),
			0..0
		))
	);
}

#[test]
fn parse_div_assign() {
	let res = parse("a /= 1");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::DivAssign(var!(a 0..1), n!(1 5)),
			0..0
		))
	);
}

#[test]
fn parse_mod_assign() {
	let res = parse("a %= 1");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::ModAssign(var!(a 0..1), n!(1 5)),
			0..0
		))
	);
}

#[test]
fn parse_for_loop_expr() {
	let res = parse("iter arr of item { print(item) }");

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::For {
				collection: ident!(arr 5..8),
				item: var!(item 12..16),
				body: code! {
					call_ident!(print 19..24 (*ident!(item 25..29)))
				},
			},
			0..0
		))
	);
}

#[test]
fn parse_for_loop_expr_spanning_multiple_lines() {
	let res = parse(indoc! {"
			iter arr of item {
				print(item)
			}"
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::For {
				collection: ident!(arr 5..8),
				item: var!(item 12..16),
				body: code! {
					call_ident!(print 20..25 (*ident!(item 26..30)))
				},
			},
			0..0
		))
	);
}

#[test]
fn parse_for_loop_over_expression() {
	let res = parse(indoc! {r#"
			iter [
				"some string"
				12345
			] of item {
				print(item)
			}"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::For {
				collection: arr![*str!("some string" 8), *n!(12345 23); 5..30],
				item: var!(item 34..38),
				body: code! {
					call_ident!(print 42..47 (*ident!(item 48..52)))
				},
			},
			0..0
		))
	);
}

#[test]
fn parse_for_loop_over_if_else_expression() {
	let res = parse(indoc! {r#"
			iter if a > 10 {
				"this"
			} else {
				["or", "this"]
			} of item {
				print(item)
			}"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::For {
				collection: Box::new(Expr::new(
					ExprKind::IfElse {
						condition: Box::new(Expr::new(
							ExprKind::BinExpr(BinExpr {
								span: (0..0).into(),
								op: BinExprKind::GreaterThan,
								lhs: ident!(a 8..9),
								rhs: n!(10 12)
							}),
							0..0
						)),
						yes: code! {
							*str!("this" 18)
						},
						no: code! {
							*arr![*str!("or" 36), *str!("this" 42); 35..49]
						}
					},
					0..0
				)),
				item: var!(item 55..59),
				body: code! {
					call_ident!(print 63..68 (*ident!(item 69..73)))
				},
			},
			0..0
		))
	);
}

#[test]
fn parse_for_loop_with_break_expression_in_it() {
	let res = parse(indoc! {"
			iter arr of x {
				if x > 10 {
					break x
				}
			}"
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::For {
				collection: ident!(arr 5..8),
				item: var!(x 12..13),
				body: code! {
					Expr::new(
						ExprKind::If {
							condition: Box::new(
								Expr::new(ExprKind::BinExpr(
									BinExpr {
										span: (0..0).into(),
										op: BinExprKind::GreaterThan,
										lhs: ident!(x 20..21),
										rhs: n!(10 24)
									}
								), 0..0)
							),
							yes: code!{
								Expr::new(
									ExprKind::Break(ident!(x 37..38)),
									0..0
								)
							}
						},
						0..0
					)
				},
			},
			0..0
		))
	);
}

#[test]
fn parse_function_expr() {
	let res = parse(indoc! {r#"
		|x, y| -> x + y"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Function(Function {
				parameters: Parameters(vec![var!(x 1..2), var!(y 4..5)]),
				body: code! {
				*ident!(x 10..11) + *ident!(y 14..15)
				}
			}),
			0..0
		))
	);
}

#[test]
fn parse_function_expr_with_block_body() {
	let res = parse(indoc! {r#"
			|x, y| -> {
				let local = 2 * x
				kok(local + y)
			}"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Function(Function {
				parameters: Parameters(vec![var!(x 1..2), var!(y 4..5)]),
				body: code! {
					Expr::new(
						ExprKind::Declare(
							var!(local 17..22),
							n!(2 25) * ident!(x 29..30)
						),
						0..0
					),
					call_ident!(kok 32..35 (*ident!(local 36..41) + *ident!(y 44..45)))
				}
			}),
			0..0
		))
	);
}

#[test]
fn parse_iife() {
	let res = parse(indoc! {r#"
			|x| -> {
				print(x)
			}(123)"#
	});

	assert_eq!(
		res,
		Ok(*call!(
			Box::new(Expr::new(ExprKind::Function(Function {
				parameters: Parameters(vec![var!(x 1..2)]),
				body: code!{
					call_ident!(print 10..15 (*ident!(x 16..17)))
				}
			}), 0..0));
			(*n!(123 21))
		))
	);
}

#[test]
fn parse_function_expression_with_zero_arguments() {
	let res = parse(indoc! {r#"
		|| -> { 123 }"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Function(Function {
				parameters: Parameters(vec![]),
				body: code! { *n!(123 8) }
			}),
			0..0
		))
	);
}

#[test]
fn parse_lambda_expression_with_zero_arguments() {
	let res = parse(indoc! {r#"
		|| -> 123"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Function(Function {
				parameters: Parameters(vec![]),
				body: code! { *n!(123 6) }
			}),
			0..0
		))
	);
}

#[test]
fn parse_nested_lambda_expression() {
	let res = parse(indoc! {r#"
		|| -> || -> || -> nil"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Function(Function {
				parameters: Parameters(vec![]),
				body: code! {
					Expr::new(
						ExprKind::Function(
							Function {
								parameters: Parameters(vec![]),
								body: code! {
									Expr::new(
										ExprKind::Function(Function {
											parameters: Parameters(vec![]),
											body: code! { *nil!(18) }
										}),
										0..0
									)
								}
							}
						),
					0..0)
				}
			}),
			0..0
		))
	);
}

#[test]
fn parse_function_expr_with_multiple_argument_delimited_by_newline() {
	let res = parse(indoc! {r#"
			|
				x
				y
			| -> {
				let local = 2 * x
				kok(local + y)
			}"#
	});

	assert_eq!(
		res,
		Ok(Expr::new(
			ExprKind::Function(Function {
				parameters: Parameters(vec![var!(x 3..4), var!(y 6..7)]),
				body: code! {
					Expr::new(
						ExprKind::Declare(
							var!(local 20..25),
							n!(2 28) * ident!(x 32..33)
						),
						0..0
					),
					call_ident!(kok 35..38 (*ident!(local 39..44) + *ident!(y 47..48)))
				}
			}),
			0..0
		))
	);
}
