use std::collections::{
	hash_map::{Entry, OccupiedEntry, VacantEntry},
	HashMap,
};

use parser::{
	ast::{
		Array, BinExpr, Boolean, Code, Expr, Function, Ident, Integer, Literal,
		Nil, StringT,
	},
	parse_code, LexError, ParseError, Token,
};

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
	ParseError(ParseError<usize, Token, LexError>),
	RuntimeError(RuntimeError),
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
	ReferenceError(ReferenceError),
	AssignmentToImmutableVariable,
	AlreadyDeclared,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Nil,
	Boolean(bool),
	Integer(i32),
	String(String),
	Array(Vec<Value>),
}

impl Value {
	fn from_literal(ex: Literal) -> Self {
		match ex {
			Literal::Nil(Nil) => Self::Nil,
			Literal::Boolean(Boolean(v)) => Self::Boolean(v),
			Literal::Integer(Integer(v)) => Self::Integer(v),
			Literal::String(StringT(v)) => Self::String(v),
		}
	}
}

struct SymbolInfo {
	mutable: bool,
	value: Value,
}

pub struct Environment {
	store: HashMap<Ident, SymbolInfo>,
}

impl Environment {
	fn new() -> Self {
		Self {
			store: HashMap::new(),
		}
	}

	fn entry(
		&mut self,
		ident: Ident,
	) -> Result<OccupiedEntry<'_, Ident, SymbolInfo>, RuntimeError> {
		match self.store.entry(ident) {
			Entry::Occupied(o) => Ok(o),
			Entry::Vacant(_) => Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier,
			)),
		}
	}

	fn entry_vacant(
		&mut self,
		ident: Ident,
	) -> Result<VacantEntry<'_, Ident, SymbolInfo>, RuntimeError> {
		match self.store.entry(ident) {
			Entry::Occupied(_) => Err(RuntimeError::AlreadyDeclared),
			Entry::Vacant(v) => Ok(v),
		}
	}

	pub fn declare(
		&mut self,
		ident: Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		self.entry_vacant(ident)?
			.insert(SymbolInfo { mutable, value });
		Ok(())
	}

	pub fn assign(
		&mut self,
		ident: Ident,
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

	pub fn read_value(&mut self, ident: Ident) -> Result<Value, RuntimeError> {
		Ok(self.entry(ident)?.get().value.clone())
	}

	pub fn free(&mut self, ident: Ident) -> Result<(), RuntimeError> {
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
			Tree::Boolean(i) => Ok(Value::Boolean(i.0)),
			Tree::Integer(i) => Ok(Value::Integer(i.0)),
			Tree::StringT(i) => Ok(Value::String(i.0)),
			Tree::Literal(i) => match i {
				Literal::Nil(i) => self.eval(Tree::Nil(i)),
				Literal::Boolean(i) => self.eval(Tree::Boolean(i)),
				Literal::Integer(i) => self.eval(Tree::Integer(i)),
				Literal::String(i) => self.eval(Tree::StringT(i)),
			},
			Tree::Ident(ident) => Ok(self.mem.read_value(ident)?),
			Tree::Array(i) => Ok(Value::Array(
				i.0 .0
					.into_iter()
					.map(|e| self.eval(Tree::Expr(e)))
					.collect::<Result<Vec<_>, RuntimeError>>()?,
			)),
			Tree::Function(_) => todo!(),
			Tree::BinExpr(i) => match i {
				BinExpr::Add(i, j) => {
					let i = self.eval(Tree::Expr(*i))?;
					let j = self.eval(Tree::Expr(*j))?;

					Ok(match (i, j) {
						(Value::Integer(i), Value::Integer(i2)) =>
							Value::Integer(i + i2),
						(Value::Integer(i), Value::String(s)) =>
							Value::String(format!("{i}{s}")),
						(Value::String(s), Value::Integer(i)) =>
							Value::String(format!("{s}{i}")),
						(Value::String(s), Value::String(s2)) =>
							Value::String(format!("{s}{s}")),
						(_, _) => panic!(),
					})
				},
				BinExpr::Sub(i, j) => {
					let Value::Integer(i) = self.eval(Tree::Expr(*i))? else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					Ok(Value::Integer(i - j))
				},
				BinExpr::Mul(i, j) => {
					let Value::Integer(i) = self.eval(Tree::Expr(*i))? else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					Ok(Value::Integer(i * j))
				},
				BinExpr::Div(i, j) => {
					let Value::Integer(i) = self.eval(Tree::Expr(*i))? else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					Ok(Value::Integer(i / j))
				},
				BinExpr::Mod(_, _) => todo!(),
				BinExpr::Equal(_, _) => todo!(),
				BinExpr::NotEqual(_, _) => todo!(),
				BinExpr::LessThan(_, _) => todo!(),
				BinExpr::GreaterThan(_, _) => todo!(),
				BinExpr::LessThanEqual(_, _) => todo!(),
				BinExpr::GreaterThanEqual(_, _) => todo!(),
				BinExpr::And(_, _) => todo!(),
				BinExpr::Or(_, _) => todo!(),
				BinExpr::Call(i, j) => {
					let Some(arg) = j.0.into_iter().nth(0) else {
						panic!()
					};
					let arg = self.eval(Tree::Expr(arg))?;
					let Expr::Ident(Ident(i)) = *i else { panic!() };
					match &i[..] {
						"print" => {
							println!("{:?}", arg);
							Ok(arg)
						},
						_ => unimplemented!(),
					}
				},
				BinExpr::Prop(_, _) => todo!(),
				BinExpr::Index(_, _) => todo!(),
			},
			Tree::Expr(i) => match i {
				Expr::Literal(i) => Ok(self.eval(Tree::Literal(i))?),
				Expr::Ident(i) => Ok(self.eval(Tree::Ident(i))?),
				Expr::UnaryMinus(i) => {
					let Value::Integer(i) = self.eval(Tree::Expr(*i))? else {
						panic!();
					};
					Ok(Value::Integer(-i))
				},
				Expr::UnaryNot(i) => {
					let Value::Boolean(i) = self.eval(Tree::Expr(*i))? else {
						panic!();
					};
					Ok(Value::Boolean(!i))
				},
				Expr::Array(i) => self.eval(Tree::Array(i)),
				Expr::Function(_) => todo!(),
				Expr::BinExpr(i) => self.eval(Tree::BinExpr(i)),
				Expr::Assign(ident, j) => {
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(j);
					self.mem.assign(ident, v.clone())?;
					Ok(v)
				},
				Expr::AddAssign(ident, j) => {
					let Value::Integer(i) =
						self.mem.read_value(ident.clone())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i + j);
					self.mem.assign(ident, v.clone())?;
					Ok(v)
				},
				Expr::SubAssign(ident, j) => {
					let Value::Integer(i) =
						self.mem.read_value(ident.clone())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i - j);
					self.mem.assign(ident, v.clone())?;
					Ok(v)
				},
				Expr::MulAssign(ident, j) => {
					let Value::Integer(i) =
						self.mem.read_value(ident.clone())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i * j);
					self.mem.assign(ident, v.clone())?;
					Ok(v)
				},
				Expr::DivAssign(ident, j) => {
					let Value::Integer(i) =
						self.mem.read_value(ident.clone())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i / j);
					self.mem.assign(ident, v.clone())?;
					Ok(v)
				},
				Expr::ModAssign(ident, j) => {
					let Value::Integer(i) =
						self.mem.read_value(ident.clone())?
					else {
						panic!()
					};
					let Value::Integer(j) = self.eval(Tree::Expr(*j))? else {
						panic!()
					};
					let v = Value::Integer(i % j);
					self.mem.assign(ident, v.clone())?;
					Ok(v)
				},
				Expr::Declare(ident, value) => {
					let value = self.eval(Tree::Expr(*value))?;
					self.mem
						.declare(ident, value.clone(), false)?;
					Ok(value)
				},
				Expr::DeclareMut(ident, value) => {
					let value = self.eval(Tree::Expr(*value))?;
					self.mem
						.declare(ident, value.clone(), true)?;
					Ok(value)
				},
				Expr::If { condition, yes } => todo!(),
				Expr::IfElse { condition, yes, no } => todo!(),
				Expr::For {
					collection,
					item,
					body,
				} => todo!(),
				Expr::Panic(_) => todo!(),
				Expr::Assert(_) => todo!(),
				Expr::Return(_) => todo!(),
				Expr::Break(_) => todo!(),
				Expr::Continue(_) => todo!(),
			},
			Tree::Code(i) => {
				let mut last = Value::Nil;
				for i in i.0 {
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

	use super::*;

	#[test]
	fn run_interpreter() {
		let mut interpreter = Interpreter::init();

		let res = interpreter.run(indoc! {r#"
			let! x = 10
			x += 20
			let y = x * 3
			print(y - x)
		"#});

		assert_eq!(res, Ok(Value::Integer(60)));
	}
}
