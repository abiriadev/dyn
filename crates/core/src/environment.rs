use std::{
	collections::{hash_map::Entry, HashMap},
	sync::{Arc, RwLock},
};

use dyn_parser::ast::{Ident, Parameters};

use crate::{ArgumentValues, ReferenceError, RuntimeError, SymbolInfo, Value};

type IndexedStack<T> = Vec<T>;

#[derive(Debug)]
struct Scope(HashMap<Ident, SymbolInfo>);

#[derive(Debug)]
pub struct Frame {
	scope_stack: IndexedStack<Scope>,
	parent: Option<Arc<RwLock<Frame>>>,
}

impl Frame {
	fn rec_lookup<F, T>(
		&mut self,
		ident: &Ident,
		cb: F,
	) -> Result<T, RuntimeError>
	where
		F: Fn(&mut SymbolInfo) -> Result<T, RuntimeError>,
	{
		for scope in self.scope_stack.iter_mut().rev() {
			if let Entry::Occupied(mut v) = scope.0.entry(ident.to_owned()) {
				return cb(v.get_mut());
			};
		}

		let Some(parent) = &self.parent else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.clone(),
				},
			));
		};

		let parent = parent.clone();
		let mut parent = parent.write().unwrap();
		parent.rec_lookup(ident, cb)
	}
}

#[derive(Debug)]
pub struct Environment {
	call_stack: IndexedStack<Arc<RwLock<Frame>>>,
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

	pub fn top_frame(&self) -> Arc<RwLock<Frame>> {
		// WARN: unwrap
		self.call_stack.last().unwrap().clone()
	}

	pub fn declare(
		&mut self,
		ident: &Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		// WARN: unwrap
		let top = self.top_frame();
		let mut top = top.write().unwrap();
		let Entry::Vacant(v) = top
			.scope_stack
			.last_mut()
			.unwrap()
			.0
			.entry(ident.to_owned())
		else {
			return Err(RuntimeError::AlreadyDeclared);
		};

		v.insert(SymbolInfo { mutable, value });

		Ok(())
	}

	pub fn assign(
		&mut self,
		ident: &Ident,
		value: Value,
	) -> Result<(), RuntimeError> {
		let top = self.top_frame();
		let mut top = top.write().unwrap();
		top.rec_lookup(ident, move |e| {
			if !e.mutable {
				return Err(RuntimeError::AssignmentToImmutableVariable);
			}

			e.value = value.clone();

			Ok(())
		})?;

		Ok(())
	}

	pub fn load(&self, ident: &Ident) -> Result<Value, RuntimeError> {
		let top = self.top_frame();
		let mut top = top.write().unwrap();

		top.rec_lookup(ident, |e| Ok(e.value.clone()))
	}

	pub fn call(
		&mut self,
		capture: Arc<RwLock<Frame>>,
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
		self.top_frame()
			.write()
			.unwrap()
			.scope_stack
			.push(Scope(HashMap::new()));
	}

	pub fn pop_scope(&mut self) {
		self.top_frame()
			.write()
			.unwrap()
			.scope_stack
			.pop();
	}
}
