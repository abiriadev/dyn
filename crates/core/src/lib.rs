use std::collections::{
	hash_map::{Entry, OccupiedEntry},
	HashMap,
};

use parser::{ast::Ident, parse, LexError, ParseError, Token, VisitMut};

pub enum InterpreterError {
	ParseError(ParseError<usize, Token, LexError>),
	RuntimeError(RuntimeError),
}

pub enum ReferenceError {
	UndefinedIdentifier,
}

pub enum RuntimeError {
	ReferenceError(ReferenceError),
	AssignmentToImmutableVariable,
}

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
		let ast = match parse(code) {
			Ok(v) => v,
			Err(e) => return Err(InterpreterError::ParseError(e)),
		};

		let mut ast = ast;

		self.visit_mut_expr(&mut ast);

		Ok(())
	}
}

impl VisitMut for Interpreter {}
