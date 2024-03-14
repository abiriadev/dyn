use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use dyn_parser::ast::{Arguments, Ident, Parameters};

use crate::{RuntimeError, SymbolInfo};

type BindTable = HashMap<Ident, SymbolInfo>;
type Arw<T> = Arc<RwLock<T>>;
type ArwFrame = Arw<Frame>;
type RuntimeResult<T> = Result<T, RuntimeError>;
type IndexedStack; = Vec<T>;

struct Scope(BindTable);

struct Frame {
	scope_stack: IndexedStack<Scope>,
	parent: Option<ArwFrame>,
}

pub struct Environment {
	call_stack: IndexedStack<ArwFrame>,
}

impl Environment {
	pub fn new() -> Self {
		todo!()
	}

	pub fn declare(ident: &Ident, value: Value, is_mut: bool) -> RuntimeResult<()> {
        todo!()
    }

	pub fn assign(ident: &Ident, value: Value) -> RuntimeResult<()> {
        todo!()
    }

	pub fn load(ident: &Ident) -> RuntimeResult<Value> {
        todo!()
    }

	pub fn call(capture: ArwFrame, parameters: Parameters, arguments: Arguments) {
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
