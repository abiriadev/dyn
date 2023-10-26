use std::collections::{
	hash_map::{Entry, OccupiedEntry},
	HashMap,
};

use parser::{ast::Ident, parse_code, LexError, ParseError, Token, VisitMut};

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
}

#[derive(Debug)]
pub enum Value {
	Integer(i32),
	String(String),
	Array(Vec<Value>),
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

	pub fn declare(
		&mut self,
		ident: Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		self.entry(ident)?
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

	pub fn free(&mut self, ident: Ident) -> Result<(), RuntimeError> {
		self.entry(ident)?.remove();
		Ok(())
	}
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

	pub fn run(&mut self, code: &str) -> Result<(), InterpreterError> {
		let ast = match parse_code(code) {
			Ok(v) => v,
			Err(e) => return Err(InterpreterError::ParseError(e)),
		};

		let mut ast = ast;

		self.visit_mut_code(&mut ast);

		Ok(())
	}
}

impl VisitMut for Interpreter {
	fn visit_mut_integer(&mut self, i: &mut parser::ast::Integer) {
		println!("integer: {i:?}");
	}

	fn visit_mut_ident(&mut self, i: &mut Ident) {
		println!("ident: {i:?}");
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

		assert_eq!(res, Ok(()));
	}
}
