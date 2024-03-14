use std::{
	collections::{
		hash_map::{Entry, VacantEntry},
		HashMap,
	},
	sync::{Arc, RwLock},
};

use dyn_parser::ast::{Arguments, Ident, Parameters};

use crate::{ReferenceError, RuntimeError, SymbolInfo, Value};

type BindTable = HashMap<Ident, SymbolInfo>;
type Arw<T> = Arc<RwLock<T>>;
type ArwFrame = Arw<Frame>;
type RuntimeResult<T> = Result<T, RuntimeError>;
type IndexedStack<T> = Vec<T>;

#[derive(Debug)]
struct Scope(BindTable);

#[derive(Debug)]
struct Frame {
	scope_stack: IndexedStack<Scope>,
	parent: Option<ArwFrame>,
}

#[derive(Debug)]
pub struct Environment {
	call_stack: IndexedStack<ArwFrame>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			call_stack: vec![Arc::new(RwLock::new(Frame {
				scope_stack: vec![Scope(HashMap::new())],
				parent: None,
			}))],
		}
	}

	pub fn declare(
		&mut self,
		ident: &Ident,
		value: Value,
		is_mut: bool,
	) -> RuntimeResult<()> {
		// WARN: unwrap
		let Entry::Vacant(v) = self
			.call_stack
			.last()
			.unwrap()
			.write()
			.unwrap()
			.scope_stack
			.last()
			.unwrap()
			.0
			.entry(ident.to_owned())
		else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.clone(),
				},
			));
		};

		v.insert(value);

		Ok(())
	}

	pub fn assign(&mut self, ident: &Ident, value: Value) -> RuntimeResult<()> {
		todo!()
	}

	pub fn load(&self, ident: &Ident) -> RuntimeResult<Value> {
		todo!()
	}

	pub fn call(
		&mut self,
		capture: ArwFrame,
		parameters: Parameters,
		arguments: Arguments,
	) {
		todo!()
	}

	pub fn ret(&mut self) {
		todo!()
	}

	pub fn push_scope(&mut self) {
		todo!()
	}

	pub fn pop_scope(&mut self) {
		todo!()
	}
}
