use std::collections::{hash_map::Entry, HashMap};

use parser::ast::Ident;

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

impl Environment {}

pub struct Interpreter {}
