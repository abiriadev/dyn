#[macro_export]
macro_rules! n {
	($n:expr) => {
		Box::new(crate::ast::Expr::Literal(
			crate::ast::Literal::Integer(crate::ast::Integer($n)),
		))
	};
}

#[macro_export]
macro_rules! ident {
	($id:ident) => {
		Box::new(crate::ast::Expr::Ident(
			crate::ast::Ident(stringify!($id).to_owned()),
		))
	};
}
