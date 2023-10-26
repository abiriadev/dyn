#![allow(unused)]

macro_rules! n {
	($n:expr) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Integer($crate::ast::Integer($n)),
		))
	};
}

pub(crate) use n;

macro_rules! ident {
	($id:ident) => {
		Box::new($crate::ast::Expr::Ident(
			$crate::ast::Ident(stringify!($id).to_owned()),
		))
	};
}

pub(crate) use ident;

macro_rules! var {
	($id:ident) => {
		$crate::ast::Ident(stringify!($id).to_owned())
	};
}

pub(crate) use var;

macro_rules! str {
	($str:expr) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::String($crate::ast::StringT($str.to_owned())),
		))
	};
}
pub(crate) use str;

macro_rules! arr {
	[$($ele:expr),* $(,)?] => {
		Box::new(
			$crate::ast::Expr::Array(
				$crate::ast::Array(
					$crate::ast::Code(vec![$($ele),*])
				)
			)
		)
	};
}
pub(crate) use arr;

macro_rules! nil {
	() => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Nil($crate::ast::Nil),
		))
	};
}

pub(crate) use nil;

macro_rules! tru {
	() => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean(true)),
		))
	};
}

pub(crate) use tru;

macro_rules! fal {
	() => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean(false)),
		))
	};
}

pub(crate) use fal;

macro_rules! code {
	{ $($ele:expr),* $(,)? } => {
		$crate::ast::Code(vec![$($ele),*])
	};
}

pub(crate) use code;

macro_rules! save_token {
	($token:expr) => {
		|lex: &mut Lexer<'s>| {
			lex.extras = Some($token);
		}
	};
}

pub(crate) use save_token;
