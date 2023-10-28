#![allow(unused)]
use box_tt::BoxNew;
use span::Span;

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

	pub fn with_span<S>(self, span: S) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			..self
		}
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringT(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Nil(Nil),
	Boolean(Boolean),
	Integer(Integer),
	String(StringT),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, BoxNew)]
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

	pub fn with_span<S>(self, span: S) -> Self
	where S: Into<Span> {
		Self {
			span: span.into(),
			..self
		}
	}

	pub fn symbol(&self) -> &str { &self.symbol }

	pub fn into_symbol(self) -> String { self.symbol }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array(pub Code);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters(pub Vec<Ident>);

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
	pub parameters: Parameters,
	pub body: Code,
}

#[derive(Debug, Clone, PartialEq, BoxNew)]
pub enum BinExpr {
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	Equal(Box<Expr>, Box<Expr>),
	NotEqual(Box<Expr>, Box<Expr>),
	LessThan(Box<Expr>, Box<Expr>),
	GreaterThan(Box<Expr>, Box<Expr>),
	LessThanEqual(Box<Expr>, Box<Expr>),
	GreaterThanEqual(Box<Expr>, Box<Expr>),
	And(Box<Expr>, Box<Expr>),
	Or(Box<Expr>, Box<Expr>),
	Prop(Box<Expr>, Box<Expr>),
	Index(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, BoxNew)]
pub enum Expr {
	Literal(Literal),
	Ident(Ident),
	UnaryMinus(Box<Expr>),
	UnaryNot(Box<Expr>),
	Array(Array),
	Function(Function),
	Call(Box<Expr>, Arguments),
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

#[derive(Debug, Clone, PartialEq)]
pub struct Code(pub Vec<Expr>);

#[cfg(test)]
use std::ops::{Add, Div, Mul, Rem, Sub};

#[cfg(test)]
impl Add for Box<Expr> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Box::new(Expr::BinExpr(BinExpr::Add(self, rhs)))
	}
}

#[cfg(test)]
impl Add for Expr {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Expr::BinExpr(BinExpr::Add(
			Box::new(self),
			Box::new(rhs),
		))
	}
}

#[cfg(test)]
impl Sub for Box<Expr> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Box::new(Expr::BinExpr(BinExpr::Sub(self, rhs)))
	}
}

#[cfg(test)]
impl Sub for Expr {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Expr::BinExpr(BinExpr::Sub(
			Box::new(self),
			Box::new(rhs),
		))
	}
}

#[cfg(test)]
impl Mul for Box<Expr> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Box::new(Expr::BinExpr(BinExpr::Mul(self, rhs)))
	}
}

#[cfg(test)]
impl Mul for Expr {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Expr::BinExpr(BinExpr::Mul(
			Box::new(self),
			Box::new(rhs),
		))
	}
}

#[cfg(test)]
impl Div for Box<Expr> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Box::new(Expr::BinExpr(BinExpr::Mod(self, rhs)))
	}
}

#[cfg(test)]
impl Div for Expr {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Expr::BinExpr(BinExpr::Div(
			Box::new(self),
			Box::new(rhs),
		))
	}
}

#[cfg(test)]
impl Rem for Box<Expr> {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		Box::new(Expr::BinExpr(BinExpr::Div(self, rhs)))
	}
}

#[cfg(test)]
impl Rem for Expr {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		Expr::BinExpr(BinExpr::Mod(
			Box::new(self),
			Box::new(rhs),
		))
	}
}
