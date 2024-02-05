use std::collections::HashMap;

use environment::Environment;
pub use error::{InterpreterError, ReferenceError, RuntimeError};
use error::{ParseError, TypeError};
use parser::{
	ast::{
		self, Array, BinExpr, BinExprKind, Boolean, Code, Expr, ExprKind,
		Function, Ident, Integer, Literal, Nil, StringT, UnaryExpr,
		UnaryExprKind,
	},
	parse_code,
};
use span::HasSpan;
use value::Record;
pub use value::{ArgumentValues, BuiltinFunction, FunctionValue, Value};

mod environment;
pub mod error;
mod value;

struct SymbolInfo {
	mutable: bool,
	value: Value,
}

enum Tree {
	Nil(Nil),
	Boolean(Boolean),
	Integer(Integer),
	StringT(StringT),
	Literal(Literal),
	Ident(Ident),
	Array(Array),
	Record(ast::Record),
	#[allow(unused)]
	Function(Function),
	UnaryExpr(UnaryExpr),
	BinExpr(BinExpr),
	Expr(Expr),
	Code(Code),
}

pub struct Interpreter {
	mem: Environment,
}

impl Interpreter {
	pub fn init() -> Self {
		Self {
			mem: Environment::new(),
		}
	}

	pub fn init_with_builtins(
		builtin: HashMap<Ident, Value>,
	) -> Result<Self, RuntimeError> {
		let mut mem = Environment::new();

		for (ident, value) in builtin {
			mem.declare(ident, value, false)?;
		}

		Ok(Self { mem })
	}

	fn parse_code(&mut self, code: &str) -> Result<Code, InterpreterError> {
		parse_code(code)
			.map_err(|e| InterpreterError::ParseError(ParseError(e)))
	}

	pub fn run(&mut self, code: &str) -> Result<Value, InterpreterError> {
		let ast = self.parse_code(code)?;

		let res = self
			.eval(Tree::Code(ast))
			.map_err(InterpreterError::RuntimeError)?;

		Ok(res)
	}

	fn eval(&mut self, ast: Tree) -> Result<Value, RuntimeError> {
		match ast {
			Tree::Nil(_) => Ok(Value::Nil),
			Tree::Boolean(i) => Ok(Value::Boolean(i.value())),
			Tree::Integer(i) => Ok(Value::Integer(i.value)),
			Tree::StringT(i) => Ok(Value::String(i.value)),
			Tree::Literal(i) => match i {
				Literal::Nil(i) => self.eval(Tree::Nil(i)),
				Literal::Boolean(i) => self.eval(Tree::Boolean(i)),
				Literal::Integer(i) => self.eval(Tree::Integer(i)),
				Literal::String(i) => self.eval(Tree::StringT(i)),
			},
			Tree::Ident(ident) => Ok(self.mem.read_value(ident.into())?),
			Tree::Array(i) => Ok(Value::Array(
				i.elements
					.into_iter()
					.map(|e| self.eval(Tree::Expr(e)))
					.collect::<Result<Vec<_>, RuntimeError>>()?,
			)),
			Tree::Record(i) => Ok(Value::Record(Record {
				fields: i.fields.into_iter().try_fold(
					HashMap::<Ident, Value>::new(),
					|mut h, (k, v)| -> Result<HashMap<_, _>, RuntimeError> {
						h.insert(k, self.eval(Tree::Expr(v))?);
						Ok(h)
					},
				)?,
			})),
			Tree::Function(_) => todo!(),
			Tree::UnaryExpr(un) => {
				let span = un.span();
				let op = un.op;
				let expr = *un.expr;

				let i = self.eval(Tree::Expr(expr))?;

				Ok(match (op, i.clone()) {
					(UnaryExprKind::Minus, Value::Integer(i)) =>
						Value::Integer(-i),
					(UnaryExprKind::Not, Value::Boolean(i)) =>
						Value::Boolean(!i),
					_ => Err(TypeError::UnaryOp {
						op,
						expr: i,
						expr_span: span,
					})?,
				})
			},
			Tree::BinExpr(bin) => {
				// let span = bin.span();
				let lhs = *bin.lhs;
				let rhs = *bin.rhs;

				let op = bin.op;

				let lhs_span = lhs.span();
				let rhs_span = rhs.span();

				let i = self.eval(Tree::Expr(lhs))?;
				let j = self.eval(Tree::Expr(rhs))?;

				Ok(match (i, op, j) {
					(
						Value::Integer(i),
						BinExprKind::Add,
						Value::Integer(j),
					) => Value::Integer(i + j),
					(Value::Integer(i), BinExprKind::Add, Value::String(s)) =>
						Value::String(format!("{i}{s}")),
					(Value::String(s), BinExprKind::Add, Value::Integer(i)) =>
						Value::String(format!("{s}{i}")),
					(Value::String(s), BinExprKind::Add, Value::String(s2)) =>
						Value::String(format!("{s}{s2}")),
					(
						Value::Integer(i),
						BinExprKind::Sub,
						Value::Integer(j),
					) => Value::Integer(i - j),
					(
						Value::Integer(i),
						BinExprKind::Mul,
						Value::Integer(j),
					) => Value::Integer(i * j),
					(
						Value::Integer(i),
						BinExprKind::Div,
						Value::Integer(j),
					) => Value::Integer(i / j),
					(
						Value::Integer(i),
						BinExprKind::Mod,
						Value::Integer(j),
					) => Value::Integer(i % j),
					(i, BinExprKind::Equal, j) => Value::Boolean(i == j),
					(i, BinExprKind::NotEqual, j) => Value::Boolean(i != j),
					(
						Value::Integer(i),
						BinExprKind::LessThan,
						Value::Integer(j),
					) => Value::Boolean(i < j),
					(
						Value::Integer(i),
						BinExprKind::GreaterThan,
						Value::Integer(j),
					) => Value::Boolean(i > j),
					(
						Value::Integer(i),
						BinExprKind::LessThanEqual,
						Value::Integer(j),
					) => Value::Boolean(i <= j),
					(
						Value::Integer(i),
						BinExprKind::GreaterThanEqual,
						Value::Integer(j),
					) => Value::Boolean(i >= j),
					(
						Value::Boolean(i),
						BinExprKind::And,
						Value::Boolean(j),
					) => Value::Boolean(i && j),
					(Value::Boolean(i), BinExprKind::Or, Value::Boolean(j)) =>
						Value::Boolean(i || j),
					(i, op, j) => Err(TypeError::BinOp {
						op,
						lhs: i,
						lhs_span,
						rhs: j,
						rhs_span,
					})?,
				})
			},
			Tree::Expr(i) => match i.kind {
				ExprKind::Literal(i) => Ok(self.eval(Tree::Literal(i))?),
				ExprKind::Ident(i) => Ok(self.eval(Tree::Ident(i))?),
				ExprKind::UnaryExpr(i) => self.eval(Tree::UnaryExpr(i)),
				ExprKind::Array(i) => self.eval(Tree::Array(i)),
				ExprKind::Record(i) => self.eval(Tree::Record(i)),
				ExprKind::Function(f) => Ok(Value::Function(
					FunctionValue::Closure {
						body: f,
						capture: self.mem.top_frame(),
					},
				)),
				ExprKind::Call(i, j) => {
					let j = ArgumentValues(
						j.0.into_iter()
							.map(|a| self.eval(Tree::Expr(a)))
							.collect::<Result<Vec<_>, RuntimeError>>()?,
					);
					let Value::Function(i) = self.eval(Tree::Expr(*i))? else {
						panic!();
					};

					match i {
						FunctionValue::Builtin(mut i) => Ok(i.call(j)),
						FunctionValue::Closure {
							body: Function { parameters, body },
							capture,
						} => {
							self.mem.call(capture, parameters, j)?;
							let v = self.eval(Tree::Code(body));
							self.mem.drop()?;
							v
						},
					}
				},
				ExprKind::Prop(i, j) => {
					let Value::Record(i) = self.eval(Tree::Expr(*i))? else {
						panic!()
					};

					Ok(i.fields
						.get(&j)
						.unwrap_or(&Value::Nil)
						.clone())
				},
				ExprKind::Index(i, j) => {
					let Value::Array(i) = self.eval(Tree::Expr(*i))? else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					if j < 0 {
						panic!("index is less than zero: {j}")
					}
					Ok(i.into_iter()
						.nth(j as usize)
						.unwrap_or(Value::Nil))
				},
				ExprKind::BinExpr(i) => self.eval(Tree::BinExpr(i)),
				ExprKind::Assign(ident, j) => {
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(j);
					self.mem
						.assign(ident.into(), v.clone())?;
					Ok(v)
				},
				ExprKind::AddAssign(ident, j) => {
					let Value::Integer(i) = self
						.mem
						.read_value(ident.clone().into())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i + j);
					self.mem
						.assign(ident.into(), v.clone())?;
					Ok(v)
				},
				ExprKind::SubAssign(ident, j) => {
					let Value::Integer(i) = self
						.mem
						.read_value(ident.clone().into())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i - j);
					self.mem
						.assign(ident.into(), v.clone())?;
					Ok(v)
				},
				ExprKind::MulAssign(ident, j) => {
					let Value::Integer(i) = self
						.mem
						.read_value(ident.clone().into())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i * j);
					self.mem
						.assign(ident.into(), v.clone())?;
					Ok(v)
				},
				ExprKind::DivAssign(ident, j) => {
					let Value::Integer(i) = self
						.mem
						.read_value(ident.clone().into())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i / j);
					self.mem
						.assign(ident.into(), v.clone())?;
					Ok(v)
				},
				ExprKind::ModAssign(ident, j) => {
					let Value::Integer(i) = self
						.mem
						.read_value(ident.clone().into())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i % j);
					self.mem
						.assign(ident.into(), v.clone())?;
					Ok(v)
				},
				ExprKind::Declare(ident, value) => {
					let value = self.eval(Tree::Expr(*value))?;
					self.mem
						.declare(ident.into(), value.clone(), false)?;
					Ok(value)
				},
				ExprKind::DeclareMut(ident, value) => {
					let value = self.eval(Tree::Expr(*value))?;
					self.mem
						.declare(ident.into(), value.clone(), true)?;
					Ok(value)
				},
				ExprKind::If { condition, yes } => {
					let Value::Boolean(condition) =
						self.eval(Tree::Expr(*condition))?
					else {
						panic!()
					};
					Ok(if condition {
						self.eval(Tree::Code(yes))?
					} else {
						Value::Nil
					})
				},
				ExprKind::IfElse { condition, yes, no } => {
					let Value::Boolean(condition) =
						self.eval(Tree::Expr(*condition))?
					else {
						panic!()
					};
					Ok(if condition {
						self.eval(Tree::Code(yes))?
					} else {
						self.eval(Tree::Code(no))?
					})
				},
				ExprKind::For {
					collection,
					item,
					body,
				} => {
					let Value::Array(collection) =
						self.eval(Tree::Expr(*collection))?
					else {
						panic!()
					};
					self.mem
						.declare(item.clone().into(), Value::Nil, true)?;
					for i in collection {
						self.mem
							.assign(item.clone().into(), i)?;
						self.eval(Tree::Code(body.clone()))?;
					}
					Ok(Value::Nil)
				},
				ExprKind::Panic(_) => todo!(),
				ExprKind::Assert(_) => todo!(),
				ExprKind::Return(_) => todo!(),
				ExprKind::Break(_) => todo!(),
				ExprKind::Continue(_) => todo!(),
			},
			Tree::Code(i) => {
				let mut last = Value::Nil;
				for i in i.stmts {
					last = self.eval(Tree::Expr(i))?;
				}
				Ok(last)
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use indoc::indoc;
	use maplit::hashmap;
	use pretty_assertions::assert_eq;

	use super::*;
	use crate::value::BuiltinFunction;

	#[derive(Clone)]
	struct Print(Vec<Value>);

	impl Print {
		fn new() -> Box<Self> { Box::new(Self(vec![])) }
	}

	impl BuiltinFunction for Print {
		fn call(&mut self, args: ArgumentValues) -> Value {
			let msg = args.0.into_iter().nth(0).unwrap();
			self.0.push(msg.clone());
			msg
		}
	}

	#[test]
	#[ignore]
	fn run_interpreter() {
		let mut interpreter = Interpreter::init_with_builtins(hashmap! {
			Ident::new_dummy("print").into() => Value::Function(
				FunctionValue::Builtin(Print::new())
			)
		})
		.unwrap();

		let res = interpreter.run(indoc! {r#"
			let! x = 10
			x += 20
			let y = x * 3
			print(y - x)
		"#});

		assert_eq!(res, Ok(Value::Integer(60)));
	}

	#[test]
	fn one_plus_one_should_be_two() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			1 + 1 == 2
		"#});

		assert_eq!(res, Ok(Value::Boolean(true)));
	}

	#[test]
	fn one_should_be_less_than_ten() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			0 < 10
		"#});

		assert_eq!(res, Ok(Value::Boolean(true)));
	}

	#[test]
	fn one_should_be_less_than_or_equal_to_ten() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			0 <= 10
		"#});

		assert_eq!(res, Ok(Value::Boolean(true)));
	}

	#[test]
	fn one_should_not_be_greater_than_ten() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			0 > 10
		"#});

		assert_eq!(res, Ok(Value::Boolean(false)));
	}

	#[test]
	fn one_should_not_be_greater_than_or_equal_to_ten() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			0 >= 10
		"#});

		assert_eq!(res, Ok(Value::Boolean(false)));
	}

	#[test]
	fn two_should_not_be_less_than_two() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			2 < 2
		"#});

		assert_eq!(res, Ok(Value::Boolean(false)));
	}

	#[test]
	fn two_should_be_less_than_or_equal_to_two() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			2 <= 2
		"#});

		assert_eq!(res, Ok(Value::Boolean(true)));
	}

	#[test]
	fn two_should_not_be_greater_than_two() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			2 > 2
		"#});

		assert_eq!(res, Ok(Value::Boolean(false)));
	}

	#[test]
	fn two_should_be_greater_than_or_equal_to_two() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			2 >= 2
		"#});

		assert_eq!(res, Ok(Value::Boolean(true)));
	}

	#[test]
	fn negative_zero_should_not_be_less_than_positive_zero() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			-0 < 0
		"#});

		assert_eq!(res, Ok(Value::Boolean(false)));
	}
}
