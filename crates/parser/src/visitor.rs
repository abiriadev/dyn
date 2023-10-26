use crate::ast::{
	Array, BinExpr, Boolean, Code, Expr, Function, Ident, Integer, Literal,
	Nil, StringT,
};

#[allow(unused)]
pub trait Visit {
	fn visit_nil(&mut self, i: &Nil) {}
	fn visit_boolean(&mut self, i: &Boolean) {}
	fn visit_integer(&mut self, i: &Integer) {}
	fn visit_string(&mut self, i: &StringT) {}
	fn visit_literal(&mut self, i: &Literal) {}
	fn visit_ident(&mut self, i: &Ident) {}
	fn visit_array(&mut self, i: &Array) {}
	fn visit_function(&mut self, i: &Function) {}
	fn visit_binexpr(&mut self, i: &BinExpr) {}
	fn visit_expr(&mut self, i: &Expr) {}
	fn visit_code(&mut self, i: &Code) {}
}
