use std::{
	collections::{
		hash_map::{Entry, OccupiedEntry, VacantEntry},
		HashMap,
	},
	fmt::{self, Debug, Display, Formatter},
	ops::Range,
};

use dyn_clone::{clone_trait_object, DynClone};
use error::{InterpreterError, ReferenceError, RuntimeError};
use miette::SourceSpan;
use parser::{
	ast::{
		Array, BinExpr, BinExprKind, Boolean, Code, Expr, ExprKind, Function,
		HasSpan, Ident, Integer, Literal, Nil, StringT,
	},
	parse_code,
};

pub mod error;

pub struct ArgumentValues(pub Vec<Value>);

pub trait BuiltinFunction: DynClone {
	fn call(&mut self, args: ArgumentValues) -> Value;
}

clone_trait_object!(BuiltinFunction);

pub enum FunctionValue {
	Builtin(Box<dyn BuiltinFunction>),
	Lambda(Function),
}

impl Debug for FunctionValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Builtin(_) => write!(f, "[BUILTIN FUNCTION]"),
			Self::Lambda(arg0) => f
				.debug_tuple("Lambda")
				.field(arg0)
				.finish(),
		}
	}
}

impl Clone for FunctionValue {
	fn clone(&self) -> Self {
		match self {
			Self::Builtin(arg0) => Self::Builtin(arg0.clone()),
			Self::Lambda(arg0) => Self::Lambda(arg0.clone()),
		}
	}
}

impl PartialEq for FunctionValue {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Builtin(l0), Self::Builtin(r0)) => unimplemented!(),
			(Self::Lambda(l0), Self::Lambda(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Nil,
	Boolean(bool),
	Integer(i32),
	String(String),
	Array(Vec<Value>),
	Function(FunctionValue),
}

impl Value {
	fn from_literal(ex: Literal) -> Self {
		match ex {
			Literal::Nil(Nil) => Self::Nil,
			Literal::Boolean(Boolean { value, .. }) => Self::Boolean(value),
			Literal::Integer(Integer { value, .. }) => Self::Integer(value),
			Literal::String(StringT { value, .. }) => Self::String(value),
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Value::Nil => "nil".to_owned(),
			Value::Boolean(i) => i.to_string(),
			Value::Integer(i) => i.to_string(),
			Value::String(i) => i.to_string(),
			Value::Array(i) => format!("{i:?}"),
			Value::Function(_) => "FUNCTION".to_owned(),
		})
	}
}

struct SymbolInfo {
	mutable: bool,
	value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedIdent(String);

impl ResolvedIdent {
	pub fn new<T>(symbol: T) -> Self
	where T: Into<String> {
		Self(symbol.into())
	}
}

impl From<Ident> for ResolvedIdent {
	fn from(value: Ident) -> Self { Self(value.into_symbol()) }
}

pub struct Environment {
	store: HashMap<ResolvedIdent, SymbolInfo>,
}

impl Environment {
	fn new() -> Self {
		Self {
			store: HashMap::new(),
		}
	}

	fn entry(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<OccupiedEntry<'_, ResolvedIdent, SymbolInfo>, RuntimeError> {
		match self.store.entry(ident) {
			Entry::Occupied(o) => Ok(o),
			Entry::Vacant(_) => Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier,
			)),
		}
	}

	fn entry_vacant(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<VacantEntry<'_, ResolvedIdent, SymbolInfo>, RuntimeError> {
		match self.store.entry(ident) {
			Entry::Occupied(_) => Err(RuntimeError::AlreadyDeclared),
			Entry::Vacant(v) => Ok(v),
		}
	}

	pub fn declare(
		&mut self,
		ident: ResolvedIdent,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		self.entry_vacant(ident)?
			.insert(SymbolInfo { mutable, value });
		Ok(())
	}

	pub fn assign(
		&mut self,
		ident: ResolvedIdent,
		value: Value,
	) -> Result<(), RuntimeError> {
		let mut e = self.entry(ident)?;
		let sym = e.get_mut();

		if !sym.mutable {
			return Err(RuntimeError::AssignmentToImmutableVariable);
		}

		sym.value = value;
		Ok(())
	}

	pub fn read_value(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<Value, RuntimeError> {
		Ok(self.entry(ident)?.get().value.clone())
	}

	pub fn free(&mut self, ident: ResolvedIdent) -> Result<(), RuntimeError> {
		self.entry(ident)?.remove();
		Ok(())
	}
}

enum Tree {
	Nil(Nil),
	Boolean(Boolean),
	Integer(Integer),
	StringT(StringT),
	Literal(Literal),
	Ident(Ident),
	Array(Array),
	Function(Function),
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
		builtin: HashMap<ResolvedIdent, Value>,
	) -> Result<Self, RuntimeError> {
		let mut mem = Environment::new();

		for (ident, value) in builtin {
			mem.declare(ident, value, false)?;
		}

		Ok(Self { mem })
	}

	pub fn run(&mut self, code: &str) -> Result<Value, InterpreterError> {
		let ast = match parse_code(code) {
			Ok(v) => v,
			Err(e) => return Err(InterpreterError::ParseError(e)),
		};

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
			Tree::Function(_) => todo!(),
			Tree::BinExpr(bin) => {
				let span = bin.span();
				let lhs = bin.lhs;
				let rhs = bin.rhs;

				match bin.op {
					BinExprKind::Add => {
						let i = self.eval(Tree::Expr(*lhs))?;
						let j = self.eval(Tree::Expr(*rhs))?;

						Ok(match (i, j) {
							(Value::Integer(i), Value::Integer(i2)) =>
								Value::Integer(i + i2),
							(Value::Integer(i), Value::String(s)) =>
								Value::String(format!("{i}{s}")),
							(Value::String(s), Value::Integer(i)) =>
								Value::String(format!("{s}{i}")),
							(Value::String(s), Value::String(s2)) =>
								Value::String(format!("{s}{s2}")),
							(_, _) => panic!(),
						})
					},
					BinExprKind::Sub => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							let rng: Range<usize> = span.into();
							return Err(RuntimeError::TypeError {
								location: rng.into(),
							});
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Integer(i - j))
					},
					BinExprKind::Mul => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Integer(i * j))
					},
					BinExprKind::Div => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Integer(i / j))
					},
					BinExprKind::Mod => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Integer(i % j))
					},
					BinExprKind::Equal => Ok(Value::Boolean(
						self.eval(Tree::Expr(*lhs))?
							== self.eval(Tree::Expr(*rhs))?,
					)),
					BinExprKind::NotEqual => Ok(Value::Boolean(
						self.eval(Tree::Expr(*lhs))?
							!= self.eval(Tree::Expr(*rhs))?,
					)),
					BinExprKind::LessThan => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Boolean(i < j))
					},
					BinExprKind::GreaterThan => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Boolean(i > j))
					},
					BinExprKind::LessThanEqual => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Boolean(i <= j))
					},
					BinExprKind::GreaterThanEqual => {
						let Value::Integer(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Integer(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Boolean(i >= j))
					},
					BinExprKind::And => {
						let Value::Boolean(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Boolean(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Boolean(i && j))
					},
					BinExprKind::Or => {
						let Value::Boolean(i) = self.eval(Tree::Expr(*lhs))?
						else {
							panic!()
						};
						let Value::Boolean(j) = self.eval(Tree::Expr(*rhs))?
						else {
							panic!()
						};
						Ok(Value::Boolean(i || j))
					},
				}
			},
			Tree::Expr(i) => match i.kind {
				ExprKind::Literal(i) => Ok(self.eval(Tree::Literal(i))?),
				ExprKind::Ident(i) => Ok(self.eval(Tree::Ident(i))?),
				ExprKind::UnaryMinus(i) => {
					let Value::Integer(i) = self.eval(Tree::Expr(*i))? else {
						panic!();
					};
					Ok(Value::Integer(-i))
				},
				ExprKind::UnaryNot(i) => {
					let Value::Boolean(i) = self.eval(Tree::Expr(*i))? else {
						panic!();
					};
					Ok(Value::Boolean(!i))
				},
				ExprKind::Array(i) => self.eval(Tree::Array(i)),
				ExprKind::Function(_) => todo!(),
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
						FunctionValue::Lambda(_) => unimplemented!(),
					}
				},
				ExprKind::Prop(_, _) => todo!(),
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
					self.mem.free(item.into())?;
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
			ResolvedIdent::new("print") => Value::Function(
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
