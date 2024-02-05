use std::{
	collections::{hash_map::Entry, HashMap},
	sync::{Arc, RwLock},
};

use parser::ast::{Ident, Parameters};

use crate::{ArgumentValues, ReferenceError, RuntimeError, SymbolInfo, Value};

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
			self.declare(k.into(), v, false)?;
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

pub struct Frame(RwLock<FrameInner>);

impl Frame {
	pub fn root() -> Arc<Self> { Arc::new(Self(FrameInner::root())) }

	pub fn new(parent: Arc<Self>) -> Arc<Self> {
		Arc::new(Self(FrameInner::new(parent)))
	}

	pub fn declare(
		self: Arc<Self>,
		ident: Ident,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		let mut inner = self.0.write().unwrap();
		let Entry::Vacant(v) = inner.table.entry(ident) else {
			return Err(RuntimeError::AlreadyDeclared)
		};

		v.insert(SymbolInfo { mutable, value });

		Ok(())
	}

	pub fn assign(
		self: Arc<Self>,
		ident: Ident,
		value: Value,
	) -> Result<(), RuntimeError> {
		let mut inner = self.0.write().unwrap();

		match inner.table.entry(ident.clone()) {
			Entry::Occupied(mut o) => {
				let ptr = o.get_mut();

				if !ptr.mutable {
					return Err(RuntimeError::AssignmentToImmutableVariable);
				}

				ptr.value = value;

				Ok(())
			},
			Entry::Vacant(_) => {
				let Some(ref p) = inner.parent else {
					return Err(RuntimeError::ReferenceError(
						ReferenceError::UndefinedIdentifier {
							ident: ident.clone(),
						},
					))
				};

				Arc::clone(p).assign(ident, value)
			},
		}
	}

	pub fn read_value(
		self: Arc<Self>,
		ident: Ident,
	) -> Result<Value, RuntimeError> {
		let inner = self.0.read().unwrap();
		match inner
			.table
			.get(&ident)
			.map(|i| i.value.clone())
		{
			Some(v) => Ok(v),
			None => inner
				.parent
				.clone()
				.ok_or(RuntimeError::ReferenceError(
					ReferenceError::UndefinedIdentifier {
						ident: ident.clone(),
					},
				))?
				.read_value(ident),
		}
	}
}

pub struct FrameInner {
	table: HashMap<Ident, SymbolInfo>,
	parent: Option<Arc<Frame>>,
}

impl FrameInner {
	fn root() -> RwLock<Self> {
		RwLock::new(Self {
			table: HashMap::new(),
			parent: None,
		})
	}

	fn new(parent: Arc<Frame>) -> RwLock<Self> {
		RwLock::new(Self {
			table: HashMap::new(),
			parent: Some(parent),
		})
	}
}
