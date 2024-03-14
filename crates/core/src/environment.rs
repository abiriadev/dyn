use std::{
	collections::{
		hash_map::{Entry, OccupiedEntry},
		HashMap,
	},
	sync::{Arc, RwLock},
};

use dyn_parser::ast::{Ident, Parameters};

use crate::{ArgumentValues, ReferenceError, RuntimeError, SymbolInfo, Value};

pub trait Memory {
	fn declare(
		&mut self,
		ident: &Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError>;

	fn assign(
		&mut self,
		ident: &Ident,
		value: Value,
	) -> Result<(), RuntimeError>;

	fn load(&self, ident: &Ident) -> Result<Value, RuntimeError>;
}

// Represents a single scope inside a function
#[derive(Debug)]
pub struct Scope(HashMap<Ident, SymbolInfo>);

impl Scope {
	fn new() -> Self {
		Self(HashMap::new())
	}

	fn new_with(init: HashMap<Ident, SymbolInfo>) -> Self {
		Self(init)
	}

	fn occupied(
		&mut self,
		ident: &Ident,
	) -> Result<OccupiedEntry<'_, Ident, SymbolInfo>, RuntimeError> {
		let Entry::Occupied(v) = self.0.entry(ident.to_owned()) else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.to_owned(),
				},
			));
		};

		Ok(v)
	}
}

// fn declare(
// 	&mut self,
// 	ident: &Ident,
// 	value: Value,
// 	mutable: bool,
// ) -> Result<(), RuntimeError> {

// Ok(())

#[derive(Debug)]
pub struct FrameInner {
	scope_stack: Vec<Scope>,
	parent: Option<Arc<Frame>>,
}

impl FrameInner {
	fn root() -> RwLock<Self> {
		RwLock::new(Self {
			scope_stack: vec![Scope::new()],
			parent: None,
		})
	}

	fn new(parent: Arc<Frame>) -> RwLock<Self> {
		RwLock::new(Self {
			scope_stack: vec![Scope::new()],
			parent: Some(parent),
		})
	}

	fn new_with(
		parent: Arc<Frame>,
		init: HashMap<Ident, SymbolInfo>,
	) -> RwLock<Self> {
		RwLock::new(Self {
			scope_stack: vec![Scope::new_with(init)],
			parent: Some(parent),
		})
	}
}

#[derive(Debug)]
pub struct Frame(RwLock<FrameInner>);

impl Frame {
	pub fn root() -> Arc<Self> {
		Arc::new(Self(FrameInner::root()))
	}

	pub fn new(parent: Arc<Self>) -> Arc<Self> {
		Arc::new(Self(FrameInner::new(parent)))
	}
}

impl Memory for Frame {
	fn declare(
		&mut self,
		ident: &Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		let Entry::Vacant(v) = self
			.0
			.write()
			.unwrap()
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

	fn assign(
		&mut self,
		ident: &Ident,
		value: Value,
	) -> Result<(), RuntimeError> {
		let mut inner = self.0.write().unwrap();

		for scope in inner.scope_stack.iter_mut().rev() {
			if let Ok(mut v) = scope.occupied(ident) {
				let ptr = v.get_mut();

				if !ptr.mutable {
					return Err(RuntimeError::AssignmentToImmutableVariable);
				}

				ptr.value = value;

				return Ok(());
			}
		}

		let Some(ref p) = inner.parent else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.clone(),
				},
			));
		};

		Arc::clone(p).assign(ident, value)
	}

	fn load(&self, ident: &Ident) -> Result<Value, RuntimeError> {
		let mut inner = self.0.read().unwrap();

		for scope in inner.scope_stack.iter_mut().rev() {
			if let Ok(v) = scope.occupied(ident) {
				return Ok(v.get().value);
			}
		}

		let Some(ref p) = inner.parent else {
			return Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier {
					ident: ident.clone(),
				},
			));
		};

		Arc::clone(p).load(ident)
	}
}

pub struct Environment {
	call_stack: Vec<Arc<Frame>>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			call_stack: vec![Frame::root()],
		}
	}

	pub fn top_frame(&self) -> Arc<Frame> {
		Arc::clone(
			self.call_stack
				.last()
				.expect("there must be at least one stack frame"),
		)
	}

	pub fn call(
		&mut self,
		capture: Arc<Frame>,
		parameters: Parameters,
		args: ArgumentValues,
	) -> Result<(), RuntimeError> {
		let frame = Frame::new(capture);

		for (k, v) in parameters
			.0
			.into_iter()
			.zip(args.0.into_iter())
		{
			self.declare(k, v, false)?;
		}

		self.call_stack.push(frame);

		Ok(())
	}

	pub fn drop(&mut self) -> Result<(), RuntimeError> {
		if self.call_stack.pop().is_none() {
			Err(RuntimeError::AlreadyDeclared)
		} else {
			Ok(())
		}
	}

	pub fn declare(
		&mut self,
		ident: Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		self.top_frame()
			.declare(ident, value, mutable)
	}

	pub fn assign(
		&mut self,
		ident: Ident,
		value: Value,
	) -> Result<(), RuntimeError> {
		self.top_frame().assign(ident, value)
	}

	pub fn read_value(&mut self, ident: Ident) -> Result<Value, RuntimeError> {
		self.top_frame().read_value(ident)
	}
}
