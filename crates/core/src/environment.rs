use std::{
	borrow::Borrow,
	cell::{Ref, RefCell},
	collections::{
		hash_map::{Entry, OccupiedEntry, VacantEntry},
		HashMap,
	},
	ops::Deref,
	rc::Rc,
};

use crate::{ReferenceError, ResolvedIdent, RuntimeError, SymbolInfo, Value};

pub struct Environment {
	call_stack: Vec<Rc<Frame>>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			call_stack: vec![Frame::root()],
		}
	}

	pub fn top_frame(&self) -> Rc<Frame> {
		Rc::clone(
			self.call_stack
				.last()
				.expect("there must be at least one stack frame"),
		)
	}

	pub fn declare(
		&mut self,
		ident: ResolvedIdent,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		self.top_frame()
			.declare(ident, value, mutable)
	}

	pub fn assign(
		&mut self,
		ident: ResolvedIdent,
		value: Value,
	) -> Result<(), RuntimeError> {
		self.top_frame().assign(ident, value)
	}

	pub fn read_value(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<Value, RuntimeError> {
		self.top_frame().read_value(ident)
	}
}

pub struct Frame(RefCell<FrameInner>);

impl Frame {
	pub fn root() -> Rc<Self> { Rc::new(Self(FrameInner::root())) }

	pub fn new(parent: Rc<Self>) -> Rc<Self> {
		Rc::new(Self(FrameInner::new(parent)))
	}

	pub fn declare(
		self: Rc<Self>,
		ident: ResolvedIdent,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		let Entry::Vacant(v) = self.0.borrow_mut().table.entry(ident) else {
			return Err(RuntimeError::AlreadyDeclared)
		};

		v.insert(SymbolInfo { mutable, value });

		Ok(())
	}

	pub fn assign(
		self: Rc<Self>,
		ident: ResolvedIdent,
		value: Value,
	) -> Result<(), RuntimeError> {
		fn r(
			frame: Rc<Frame>,
			ident: ResolvedIdent,
		) -> Result<
			OccupiedEntry<'static, ResolvedIdent, SymbolInfo>,
			RuntimeError,
		> {
			let inner = frame.deref().0.borrow_mut();
			match inner.table.entry(ident) {
				Entry::Occupied(o) => Ok(o),
				Entry::Vacant(_) => {
					let Some(p) = inner.parent else {
						return Err(RuntimeError::ReferenceError(
							ReferenceError::UndefinedIdentifier,
						))
					};

					return r(p, ident);
				},
			}
		}

		let entry = r(self, ident)?.get_mut();

		if !entry.mutable {
			return Err(RuntimeError::AssignmentToImmutableVariable)
		}

		entry.value = value;

		Ok(())
	}

	pub fn read_value(
		self: Rc<Self>,
		ident: ResolvedIdent,
	) -> Result<Value, RuntimeError> {
		match self
			.0
			.borrow()
			.table
			.get(&ident)
			.map(|i| i.value)
		{
			Some(v) => Ok(v),
			None => self
				.0
				.borrow()
				.parent
				.ok_or(RuntimeError::ReferenceError(
					ReferenceError::UndefinedIdentifier,
				))?
				.read_value(ident),
		}
	}
}

pub struct FrameInner {
	table: HashMap<ResolvedIdent, SymbolInfo>,
	parent: Option<Rc<Frame>>,
}

impl FrameInner {
	fn root() -> RefCell<Self> {
		RefCell::new(Self {
			table: HashMap::new(),
			parent: None,
		})
	}

	fn new(parent: Rc<Frame>) -> RefCell<Self> {
		RefCell::new(Self {
			table: HashMap::new(),
			parent: Some(parent),
		})
	}
}
