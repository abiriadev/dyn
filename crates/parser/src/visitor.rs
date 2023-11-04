use crate::ast::{
	Array, BinExpr, Boolean, Code, Expr, ExprKind, Function, Ident, Integer,
	Literal, Nil, StringT,
};

pub trait Visit {
	#[allow(unused)]
	fn visit_nil(&mut self, i: &Nil) {}

	#[allow(unused)]
	fn visit_boolean(&mut self, i: &Boolean) {}

	#[allow(unused)]
	fn visit_integer(&mut self, i: &Integer) {}

	#[allow(unused)]
	fn visit_string(&mut self, i: &StringT) {}

	fn visit_literal(&mut self, i: &Literal) {
		match i {
			Literal::Nil(i) => self.visit_nil(i),
			Literal::Boolean(i) => self.visit_boolean(i),
			Literal::Integer(i) => self.visit_integer(i),
			Literal::String(i) => self.visit_string(i),
		}
	}

	#[allow(unused)]
	fn visit_ident(&mut self, i: &Ident) {}

	fn visit_array(&mut self, i: &Array) {
		for i in &i.elements {
			self.visit_expr(i);
		}
	}

	fn visit_function(&mut self, i: &Function) {
		for i in &i.parameters.0 {
			self.visit_ident(i);
		}
		self.visit_code(&i.body);
	}

	fn visit_binexpr(&mut self, i: &BinExpr) {
		self.visit_expr(&i.lhs);
		self.visit_expr(&i.rhs);
	}

	fn visit_expr(&mut self, i: &Expr) {
		match &i.kind {
			ExprKind::Literal(i) => self.visit_literal(i),
			ExprKind::Ident(i) => self.visit_ident(i),
			ExprKind::UnaryMinus(i) => self.visit_expr(i),
			ExprKind::UnaryNot(i) => self.visit_expr(i),
			ExprKind::Array(i) => self.visit_array(i),
			ExprKind::Function(i) => self.visit_function(i),
			ExprKind::Call(i, j) => {
				for j in &j.0 {
					self.visit_expr(j);
				}
				self.visit_expr(i);
			},
			ExprKind::Prop(i, j) => {
				self.visit_expr(i);
				self.visit_ident(j);
			},
			ExprKind::Index(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			ExprKind::BinExpr(i) => self.visit_binexpr(i),
			ExprKind::Assign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::AddAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::SubAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::MulAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::DivAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::ModAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::Declare(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::DeclareMut(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			ExprKind::If { condition, yes } => {
				self.visit_expr(condition);
				self.visit_code(yes);
			},
			ExprKind::IfElse { condition, yes, no } => {
				self.visit_expr(condition);
				self.visit_code(yes);
				self.visit_code(no);
			},
			ExprKind::For {
				collection,
				item,
				body,
			} => {
				self.visit_expr(collection);
				self.visit_ident(item);
				self.visit_code(body);
			},
			ExprKind::Panic(i) => self.visit_expr(i),
			ExprKind::Assert(i) => self.visit_expr(i),
			ExprKind::Return(i) => self.visit_expr(i),
			ExprKind::Break(i) => self.visit_expr(i),
			ExprKind::Continue(i) => self.visit_expr(i),
		}
	}

	fn visit_code(&mut self, i: &Code) {
		for i in &i.stmts {
			self.visit_expr(i);
		}
	}
}

pub trait VisitMut {
	#[allow(unused)]
	fn visit_mut_nil(&mut self, i: &mut Nil) {}

	#[allow(unused)]
	fn visit_mut_boolean(&mut self, i: &mut Boolean) {}

	#[allow(unused)]
	fn visit_mut_integer(&mut self, i: &mut Integer) {}

	#[allow(unused)]
	fn visit_mut_string(&mut self, i: &mut StringT) {}

	fn visit_mut_literal(&mut self, i: &mut Literal) {
		match i {
			Literal::Nil(i) => self.visit_mut_nil(i),
			Literal::Boolean(i) => self.visit_mut_boolean(i),
			Literal::Integer(i) => self.visit_mut_integer(i),
			Literal::String(i) => self.visit_mut_string(i),
		}
	}

	#[allow(unused)]
	fn visit_mut_ident(&mut self, i: &mut Ident) {}

	fn visit_mut_array(&mut self, i: &mut Array) {
		for i in &mut i.elements {
			self.visit_mut_expr(i);
		}
	}

	fn visit_mut_function(&mut self, i: &mut Function) {
		for i in &mut i.parameters.0 {
			self.visit_mut_ident(i);
		}
		self.visit_mut_code(&mut i.body);
	}

	fn visit_mut_binexpr(&mut self, i: &mut BinExpr) {
		self.visit_mut_expr(&mut i.lhs);
		self.visit_mut_expr(&mut i.rhs);
	}

	fn visit_mut_expr(&mut self, i: &mut Expr) {
		match &mut i.kind {
			ExprKind::Literal(i) => self.visit_mut_literal(i),
			ExprKind::Ident(i) => self.visit_mut_ident(i),
			ExprKind::UnaryMinus(i) => self.visit_mut_expr(i),
			ExprKind::UnaryNot(i) => self.visit_mut_expr(i),
			ExprKind::Array(i) => self.visit_mut_array(i),
			ExprKind::Function(i) => self.visit_mut_function(i),
			ExprKind::Call(i, j) => {
				for j in &mut j.0 {
					self.visit_mut_expr(j);
				}
				self.visit_mut_expr(i);
			},
			ExprKind::Prop(i, j) => {
				self.visit_mut_expr(i);
				self.visit_mut_ident(j);
			},
			ExprKind::Index(i, j) => {
				self.visit_mut_expr(i);
				self.visit_mut_expr(j);
			},
			ExprKind::BinExpr(i) => self.visit_mut_binexpr(i),
			ExprKind::Assign(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::AddAssign(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::SubAssign(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::MulAssign(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::DivAssign(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::ModAssign(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::Declare(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::DeclareMut(i, j) => {
				self.visit_mut_ident(i);
				self.visit_mut_expr(j);
			},
			ExprKind::If { condition, yes } => {
				self.visit_mut_expr(condition);
				self.visit_mut_code(yes);
			},
			ExprKind::IfElse { condition, yes, no } => {
				self.visit_mut_expr(condition);
				self.visit_mut_code(yes);
				self.visit_mut_code(no);
			},
			ExprKind::For {
				collection,
				item,
				body,
			} => {
				self.visit_mut_expr(collection);
				self.visit_mut_ident(item);
				self.visit_mut_code(body);
			},
			ExprKind::Panic(i) => self.visit_mut_expr(i),
			ExprKind::Assert(i) => self.visit_mut_expr(i),
			ExprKind::Return(i) => self.visit_mut_expr(i),
			ExprKind::Break(i) => self.visit_mut_expr(i),
			ExprKind::Continue(i) => self.visit_mut_expr(i),
		}
	}

	fn visit_mut_code(&mut self, i: &mut Code) {
		for i in &mut i.stmts {
			self.visit_mut_expr(i);
		}
	}
}
