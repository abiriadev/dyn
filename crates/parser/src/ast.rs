pub struct Nil;

pub struct Boolean(bool);

pub struct Integer(i32);

pub struct StringT(String);

pub enum Literal {
	Nil(Nil),
	Boolean(Boolean),
	Integer(Integer),
	String(StringT),
}

pub struct Ident(String);

pub struct Array(Code);

pub struct Function {
	name: Ident,
	args: Vec<Ident>,
	body: Code,
}

pub enum BinExpr {
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	Assign(Box<Expr>, Box<Expr>),
	AddAssign(Box<Expr>, Box<Expr>),
	SubAssign(Box<Expr>, Box<Expr>),
	MulAssign(Box<Expr>, Box<Expr>),
	DivAssign(Box<Expr>, Box<Expr>),
	ModAssign(Box<Expr>, Box<Expr>),
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
	Declare(Box<Expr>, Box<Expr>),
	DeclareMut(Box<Expr>, Box<Expr>),
}

pub enum Expr {
	Literal(Literal),
	Ident(Ident),
	Array(Array),
	Function(Function),
	BinExpr(BinExpr),
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

pub struct Code(Vec<Expr>);
