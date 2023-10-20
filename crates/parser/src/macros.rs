#[macro_export]
macro_rules! n {
	($n:expr) => {
		Box::new(crate::ast::Expr::Literal(
			crate::ast::Literal::Integer(crate::ast::Integer($n)),
		))
	};
}
