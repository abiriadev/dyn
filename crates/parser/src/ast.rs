#![allow(unused)]
use std::{collections::HashMap, hash::Hash};

use box_tt::BoxNew;
use span::{HasSpan, Span};
use strum::AsRefStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nil {
	span: Span,
}

impl Nil {
	pub fn new<S>(span: S) -> Self
	where S: Into<Span> {
		Self { span: span.into() }
	}

	pub fn new_dummy() -> Self {
		Self {
			span: Span::DUMMY_SPAN,
		}
	}
}

impl HasSpan for Nil {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
	span: Span,
	pub value: bool,
}

impl Boolean {
	pub fn new<S>(value: bool, span: S) -> Self
	where S: Into<Span> {
		Self {
			value,
			span: span.into(),
		}
	}

	pub fn new_dummy(value: bool) -> Self {
		Self {
			value,
			span: Span::DUMMY_SPAN,
		}
	}

	pub fn value(&self) -> bool { self.value }
}

impl HasSpan for Boolean {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
	span: Span,
	pub value: i32,
}

impl Integer {
	pub fn new<S>(value: i32, span: S) -> Self
	where S: Into<Span> {
		Self {
			value,
			span: span.into(),
		}
	}

	pub fn new_dummy(value: i32) -> Self {
		Self {
			value,
			span: Span::DUMMY_SPAN,
		}
	}
}

impl HasSpan for Integer {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringT {
	span: Span,
	pub value: String,
}

impl StringT {
	pub fn new<T, S>(value: T, span: S) -> Self
	where
		T: Into<String>,
		S: Into<Span>, {
		Self {
			span: span.into(),
			value: value.into(),
		}
	}

	pub fn new_dummy<T>(value: T) -> Self
	where T: Into<String> {
		Self {
			span: Span::DUMMY_SPAN,
			value: value.into(),
		}
	}

	pub fn into_string(self) -> String { self.value }
}

impl HasSpan for StringT {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Nil(Nil),
	Boolean(Boolean),
	Integer(Integer),
	String(StringT),
}

#[derive(Debug, Clone, Eq, BoxNew)]
pub struct Ident {
	span: Span,
	symbol: String,
}

impl Ident {
	pub fn new<T, S>(symbol: T, span: S) -> Self
	where
		T: Into<String>,
		S: Into<Span>, {
		Self {
			span: span.into(),
			symbol: symbol.into(),
		}
	}

	pub fn new_dummy<T>(symbol: T) -> Self
	where T: Into<String> {
		Self {
			span: Span::DUMMY_SPAN,
			symbol: symbol.into(),
		}
	}

	pub fn symbol(&self) -> &str { &self.symbol }

	pub fn into_symbol(self) -> String { self.symbol }
}

impl PartialEq for Ident {
	fn eq(&self, other: &Self) -> bool { self.symbol == other.symbol }
}

impl Hash for Ident {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.symbol.hash(state);
	}
}

impl HasSpan for Ident {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
	span: Span,
	pub elements: Vec<Expr>,
}

impl Array {
	pub fn new<S>(elements: Vec<Expr>, span: S) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			elements,
		}
	}

	pub fn new_dummy(elements: Vec<Expr>) -> Self {
		Self {
			span: Span::DUMMY_SPAN,
			elements,
		}
	}
}

impl HasSpan for Array {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
	pub span: Span,
	pub fields: Vec<(Ident, Expr)>,
}

impl HasSpan for Record {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters(pub Vec<Ident>);

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
	pub parameters: Parameters,
	pub body: Code,
}

#[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
pub enum UnaryExprKind {
	#[strum(serialize = "-")]
	Minus,

	#[strum(serialize = "!")]
	Not,
}

#[derive(Debug, Clone, PartialEq, BoxNew)]
pub struct UnaryExpr {
	span: Span,
	pub op: UnaryExprKind,
	pub expr: Box<Expr>,
}

impl HasSpan for UnaryExpr {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum BinExprKind {
	#[strum(serialize = "+")]
	Add,

	#[strum(serialize = "-")]
	Sub,

	#[strum(serialize = "*")]
	Mul,

	#[strum(serialize = "/")]
	Div,

	#[strum(serialize = "%")]
	Mod,

	#[strum(serialize = "==")]
	Equal,

	#[strum(serialize = "!=")]
	NotEqual,

	#[strum(serialize = "<")]
	LessThan,

	#[strum(serialize = ">")]
	GreaterThan,

	#[strum(serialize = "<=")]
	LessThanEqual,

	#[strum(serialize = ">=")]
	GreaterThanEqual,

	#[strum(serialize = "&&")]
	And,

	#[strum(serialize = "||")]
	Or,
}

#[derive(Debug, Clone, PartialEq, BoxNew)]
pub struct BinExpr {
	span: Span,
	pub op: BinExprKind,
	pub lhs: Box<Expr>,
	pub rhs: Box<Expr>,
}

impl BinExpr {
	pub fn new<S>(op: BinExprKind, lhs: Expr, rhs: Expr, span: S) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			op,
			lhs: Box::new(lhs),
			rhs: Box::new(rhs),
		}
	}
}

impl HasSpan for BinExpr {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[derive(Debug, Clone, PartialEq, BoxNew)]
pub enum ExprKind {
	Literal(Literal),
	Ident(Ident),
	Array(Array),
	Record(Record),
	Function(Function),
	Call(Box<Expr>, Arguments),
	Prop(Box<Expr>, Ident),
	Index(Box<Expr>, Box<Expr>),
	UnaryExpr(UnaryExpr),
	BinExpr(BinExpr),
	Assign(Ident, Box<Expr>),
	AddAssign(Ident, Box<Expr>),
	SubAssign(Ident, Box<Expr>),
	MulAssign(Ident, Box<Expr>),
	DivAssign(Ident, Box<Expr>),
	ModAssign(Ident, Box<Expr>),
	Declare(Ident, Box<Expr>),
	DeclareMut(Ident, Box<Expr>),
	If {
		condition: Box<Expr>,
		yes: Code,
	},
	IfElse {
		condition: Box<Expr>,
		yes: Code,
		no: Code,
	},
	For {
		collection: Box<Expr>,
		item: Ident,
		body: Code,
	},
	Panic(Box<Expr>),
	Assert(Box<Expr>),
	Return(Box<Expr>),
	Break(Box<Expr>),
	Continue(Box<Expr>),
}

impl From<BinExpr> for ExprKind {
	fn from(value: BinExpr) -> Self { Self::BinExpr(value) }
}

#[derive(Debug, Clone, PartialEq, BoxNew)]
pub struct Expr {
	span: Span,
	pub kind: ExprKind,
}

impl Expr {
	pub fn new<S>(kind: ExprKind, span: S) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			kind,
		}
	}

	pub fn new_dummy(kind: ExprKind) -> Self {
		Self {
			span: Span::DUMMY_SPAN,
			kind,
		}
	}

	pub fn new_lalr_binexpr(
		(start, (lhs, op, rhs), end): (usize, (Expr, BinExprKind, Expr), usize),
	) -> Self {
		Self {
			span: (start..end).into(),
			kind: ExprKind::BinExpr(BinExpr {
				span: (start..end).into(),
				op,
				lhs: Box::new(lhs),
				rhs: Box::new(rhs),
			}),
		}
	}

	pub fn new_lalr_unaryexpr(
		(start, (op, expr), end): (usize, (UnaryExprKind, Expr), usize),
	) -> Self {
		Self {
			span: (start..end).into(),
			kind: ExprKind::UnaryExpr(UnaryExpr {
				span: (start..end).into(),
				op,
				expr: Box::new(expr),
			}),
		}
	}
}

impl HasSpan for Expr {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

impl From<BinExpr> for Expr {
	fn from(value: BinExpr) -> Self {
		Self {
			span: value.span,
			kind: value.into(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
	span: Span,
	pub stmts: Vec<Expr>,
}

impl Code {
	pub fn new<S>(stmts: Vec<Expr>, span: S) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			stmts,
		}
	}

	pub fn new_dummy(stmts: Vec<Expr>) -> Self {
		Self {
			span: Span::DUMMY_SPAN,
			stmts,
		}
	}
}

impl HasSpan for Code {
	fn span(&self) -> Span { self.span }

	fn set_span<S>(&mut self, span: S)
	where S: Into<Span> {
		self.span = span.into();
	}
}

#[cfg(test)]
use std::ops::{Add, Div, Mul, Rem, Sub};

#[cfg(test)]
impl Add for Box<Expr> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Box::new(Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Add,
				lhs: self,
				rhs,
			}),
		})
	}
}

#[cfg(test)]
impl Add for Expr {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Add,
				lhs: Box::new(self),
				rhs: Box::new(rhs),
			}),
		}
	}
}

#[cfg(test)]
impl Sub for Box<Expr> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Box::new(Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Sub,
				lhs: self,
				rhs,
			}),
		})
	}
}

#[cfg(test)]
impl Sub for Expr {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Sub,
				lhs: Box::new(self),
				rhs: Box::new(rhs),
			}),
		}
	}
}

#[cfg(test)]
impl Mul for Box<Expr> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Box::new(Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Mul,
				lhs: self,
				rhs,
			}),
		})
	}
}

#[cfg(test)]
impl Mul for Expr {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Mul,
				lhs: Box::new(self),
				rhs: Box::new(rhs),
			}),
		}
	}
}

#[cfg(test)]
impl Div for Box<Expr> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Box::new(Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Div,
				lhs: self,
				rhs,
			}),
		})
	}
}

#[cfg(test)]
impl Div for Expr {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Div,
				lhs: Box::new(self),
				rhs: Box::new(rhs),
			}),
		}
	}
}

#[cfg(test)]
impl Rem for Box<Expr> {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		Box::new(Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Mod,
				lhs: self,
				rhs,
			}),
		})
	}
}

#[cfg(test)]
impl Rem for Expr {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		Expr {
			span: self.span + rhs.span,
			kind: ExprKind::BinExpr(BinExpr {
				span: self.span + rhs.span,
				op: BinExprKind::Mod,
				lhs: Box::new(self),
				rhs: Box::new(rhs),
			}),
		}
	}
}
