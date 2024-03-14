use std::{
	collections::{
		hash_map::{Entry, OccupiedEntry},
		HashMap,
	},
	sync::{Arc, RwLock},
};

use dyn_parser::ast::{Arguments, Ident, Parameters};

use crate::{ArgumentValues, ReferenceError, RuntimeError, SymbolInfo, Value};

type BindTable = HashMap<Ident, SymbolInfo>;
type Arw<T> = Arc<RwLock<T>>;
type ArwFrame = Arw<Frame>;
type RuntimeResult<T> = Result<T, RuntimeError>;
type IndexedStack<T> = Vec<T>;
type OEntry<'a> = OccupiedEntry<'a, Ident, SymbolInfo>;

#[derive(Debug)]
struct Scope(BindTable);

impl Scope {
	fn occupied(&mut self, ident: &Ident) -> RuntimeResult<OEntry<'_>> {
		let Entry::Occupied(v) = self.0.entry(ident.to_owned()) else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.clone(),
				},
			));
		};

		Ok(v)
	}
}

#[derive(Debug)]
pub struct Frame {
	scope_stack: IndexedStack<Scope>,
	parent: Option<ArwFrame>,
}

impl Frame {
	fn rec_lookup(&mut self, ident: &Ident) -> RuntimeResult<OEntry<'_>> {
		for scope in self.scope_stack.iter_mut().rev() {
			if let Ok(v) = scope.occupied(ident) {
				return Ok(v);
			}
		}

		let Some(parent) = &self.parent else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.clone(),
				},
			));
		};

		parent
			.write()
			.unwrap()
			.rec_lookup(ident)
	}
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

	pub fn top_frame(&self) -> ArwFrame {
		self.call_stack.last().unwrap().clone()
	}

	pub fn declare(
		&mut self,
		ident: &Ident,
		value: Value,
		mutable: bool,
	) -> RuntimeResult<()> {
		// WARN: unwrap
		let Entry::Vacant(v) = self
			.call_stack
			.last()
			.unwrap()
			.read()
			.unwrap()
			.scope_stack
			.last()
			.unwrap()
			.0
			.entry(ident.to_owned())
		else {
			return Err(RuntimeError::AlreadyDeclared);
		};

		v.insert(SymbolInfo { mutable, value });

		Ok(())
	}

	pub fn assign(&mut self, ident: &Ident, value: Value) -> RuntimeResult<()> {
		let e = self
			.call_stack
			.last()
			.unwrap()
			.write()
			.unwrap()
			.rec_lookup(ident)?
			.get_mut();

		if !e.mutable {
			return Err(RuntimeError::AssignmentToImmutableVariable);
		}

		Ok(())
	}

	pub fn load(&self, ident: &Ident) -> RuntimeResult<Value> {
		Ok(self
			.call_stack
			.last()
			.unwrap()
			.read()
			.unwrap()
			.rec_lookup(ident)?
			.get()
			.value)
	}

	pub fn call(
		&mut self,
		capture: ArwFrame,
		parameters: Parameters,
		arguments: ArgumentValues,
	) {
		self.call_stack
			.push(Arc::new(RwLock::new(Frame {
				scope_stack: vec![Scope(
					parameters
						.0
						.into_iter()
						.zip(
							arguments
								.0
								.into_iter()
								.map(|value| SymbolInfo {
									mutable: false,
									value,
								}),
						)
						.collect::<HashMap<_, _>>(),
				)],
				parent: Some(capture),
			})));
	}

	pub fn ret(&mut self) {
		self.call_stack.pop();
	}

	pub fn push_scope(&mut self) {
		self.call_stack
			.last()
			.unwrap()
			.write()
			.unwrap()
			.scope_stack
			.push(Scope(HashMap::new()));
	}

	pub fn pop_scope(&mut self) {
		self.call_stack
			.last()
			.unwrap()
			.write()
			.unwrap()
			.scope_stack
			.pop();
	}
}
