use std::{
	collections::{
		hash_map::{Entry, OccupiedEntry, VacantEntry},
		HashMap,
	},
	rc::Rc,
};

use crate::{ReferenceError, ResolvedIdent, RuntimeError, SymbolInfo, Value};

pub struct Environment {
	head: Rc<Frame>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			head: Frame::root(),
		}
	}
}

pub struct Frame {
	table: HashMap<ResolvedIdent, SymbolInfo>,
	parent: Option<Rc<Self>>,
}

impl Frame {
	pub fn root() -> Rc<Self> {
		Rc::new(Self {
			table: HashMap::new(),
			parent: None,
		})
	}

	pub fn new(parent: Rc<Self>) -> Rc<Self> {
		Rc::new(Self {
			table: HashMap::new(),
			parent: Some(parent),
		})
	}

	fn entry(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<OccupiedEntry<'_, ResolvedIdent, SymbolInfo>, RuntimeError> {
		match self.table.entry(ident) {
			Entry::Occupied(o) => Ok(o),
			Entry::Vacant(_) => Err(RuntimeError::ReferenceError(
				ReferenceError::UndefinedIdentifier,
			)),
		}
	}

	fn entry_vacant(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<VacantEntry<'_, ResolvedIdent, SymbolInfo>, RuntimeError> {
		match self.table.entry(ident) {
			Entry::Occupied(_) => Err(RuntimeError::AlreadyDeclared),
			Entry::Vacant(v) => Ok(v),
		}
	}

	pub fn declare(
		&mut self,
		ident: ResolvedIdent,
		value: Value,
		mutable: bool,
	) -> Result<(), RuntimeError> {
		self.entry_vacant(ident)?
			.insert(SymbolInfo { mutable, value });
		Ok(())
	}

	pub fn assign(
		&mut self,
		ident: ResolvedIdent,
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

	pub fn read_value(
		&mut self,
		ident: ResolvedIdent,
	) -> Result<Value, RuntimeError> {
		Ok(self.entry(ident)?.get().value.clone())
	}

	pub fn free(&mut self, ident: ResolvedIdent) -> Result<(), RuntimeError> {
		self.entry(ident)?.remove();
		Ok(())
	}
}
