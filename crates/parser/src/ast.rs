use box_tt::BoxNew;

#[derive(Debug, PartialEq, Eq)]
pub struct Nil;

#[derive(Debug, PartialEq, Eq)]
pub struct Boolean(pub bool);

#[derive(Debug, PartialEq, Eq)]
pub struct Integer(pub i32);

#[derive(Debug, PartialEq, Eq)]
pub struct StringT(pub String);

#[derive(Debug, PartialEq)]
pub enum Literal {
	Nil(Nil),
	Boolean(Boolean),
	Integer(Integer),
	String(StringT),
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq)]
pub struct Array(pub Code);

#[derive(Debug, PartialEq)]
pub struct Function {
	name: Ident,
	args: Vec<Ident>,
	body: Code,
}

#[derive(Debug, PartialEq, BoxNew)]
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
	Not(Box<Expr>, Box<Expr>),
	And(Box<Expr>, Box<Expr>),
	Or(Box<Expr>, Box<Expr>),
	Call(Box<Expr>, Box<Expr>),
	Prop(Box<Expr>, Box<Expr>),
	Index(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, BoxNew)]
pub enum Expr {
	Literal(Literal),
	Ident(Ident),
	Array(Array),
	Function(Function),
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
	},
	Panic(Box<Expr>),
	Assert(Box<Expr>),
	Return(Box<Expr>),
	Break(Box<Expr>),
	Continue(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub struct Code(Vec<Expr>);
