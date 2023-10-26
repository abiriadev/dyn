use crate::ast::{
	Array, BinExpr, Boolean, Code, Expr, Function, Ident, Integer, Literal,
	Nil, StringT,
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
		for i in &i.0 .0 {
			self.visit_expr(i);
		}
	}

	fn visit_function(&mut self, i: &Function) {
		for i in &i.args {
			self.visit_ident(i);
		}
		self.visit_code(&i.body);
	}

	fn visit_binexpr(&mut self, i: &BinExpr) {
		match i {
			BinExpr::Add(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Sub(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Mul(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Div(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Mod(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Equal(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::NotEqual(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::LessThan(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::GreaterThan(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::LessThanEqual(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::GreaterThanEqual(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::And(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Or(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Call(i, j) => {
				self.visit_expr(i);
				self.visit_code(j);
			},
			BinExpr::Prop(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
			BinExpr::Index(i, j) => {
				self.visit_expr(i);
				self.visit_expr(j);
			},
		}
	}

	fn visit_expr(&mut self, i: &Expr) {
		match i {
			Expr::Literal(i) => self.visit_literal(i),
			Expr::Ident(i) => self.visit_ident(i),
			Expr::UnaryMinus(i) => self.visit_expr(i),
			Expr::UnaryNot(i) => self.visit_expr(i),
			Expr::Array(i) => self.visit_array(i),
			Expr::Function(i) => self.visit_function(i),
			Expr::BinExpr(i) => self.visit_binexpr(i),
			Expr::Assign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::AddAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::SubAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::MulAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::DivAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::ModAssign(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::Declare(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::DeclareMut(i, j) => {
				self.visit_ident(i);
				self.visit_expr(j);
			},
			Expr::If { condition, yes } => {
				self.visit_expr(condition);
				self.visit_code(yes);
			},
			Expr::IfElse { condition, yes, no } => {
				self.visit_expr(condition);
				self.visit_code(yes);
				self.visit_code(no);
			},
			Expr::For {
				collection,
				item,
				body,
			} => {
				self.visit_expr(collection);
				self.visit_ident(item);
				self.visit_code(body);
			},
			Expr::Panic(i) => self.visit_expr(i),
			Expr::Assert(i) => self.visit_expr(i),
			Expr::Return(i) => self.visit_expr(i),
			Expr::Break(i) => self.visit_expr(i),
			Expr::Continue(i) => self.visit_expr(i),
		}
	}

	fn visit_code(&mut self, i: &Code) {
		for i in &i.0 {
			self.visit_expr(i);
		}
	}
}
