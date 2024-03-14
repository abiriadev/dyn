use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use dyn_parser::ast::{Arguments, Ident, Parameters};

use crate::{RuntimeError, SymbolInfo, Value};

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
		ident: &Ident,
		value: Value,
		is_mut: bool,
	) -> RuntimeResult<()> {
		todo!()
	}

	pub fn assign(ident: &Ident, value: Value) -> RuntimeResult<()> {
		todo!()
	}

	pub fn load(ident: &Ident) -> RuntimeResult<Value> {
		todo!()
	}

	pub fn call(
		capture: ArwFrame,
		parameters: Parameters,
		arguments: Arguments,
	) {
		todo!()
	}

	pub fn ret() {
		todo!()
	}

	pub fn push_scope() {
		todo!()
	}

	pub fn pop_scope() {
		todo!()
	}
}
