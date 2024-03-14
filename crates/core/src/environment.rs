use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use dyn_parser::ast::Ident;

use crate::SymbolInfo;

type BindTable = HashMap<Ident, SymbolInfo>;
type Arw<T> = Arc<RwLock<T>>;

struct Scope(BindTable);

struct Frame {
	scope_stack: Vec<Scope>,
	parent: Option<Arw<Frame>>,
}

struct Environment {
	call_stack: Arw<Frame>,
}
