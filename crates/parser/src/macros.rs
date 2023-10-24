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

#[macro_export]
macro_rules! str {
	($str:expr) => {
		Box::new(crate::ast::Expr::Literal(
			crate::ast::Literal::String(crate::ast::StringT($str.to_owned())),
		))
	};
}

#[macro_export]
macro_rules! arr {
	[$($ele:expr),* $(,)?] => {
		Box::new(
			crate::ast::Expr::Array(
				crate::ast::Array(
					crate::ast::Code(vec![$($ele),*])
				)
			)
		)
	};
}

#[macro_export]
macro_rules! tru {
	() => {
		Box::new(crate::ast::Expr::Literal(
			crate::ast::Literal::Boolean(crate::ast::Boolean(true)),
		))
	};
}

#[macro_export]
macro_rules! fal {
	() => {
		Box::new(crate::ast::Expr::Literal(
			crate::ast::Literal::Boolean(crate::ast::Boolean(false)),
		))
	};
}
