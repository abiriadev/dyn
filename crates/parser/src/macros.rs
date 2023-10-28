#![allow(unused)]

macro_rules! n {
	($n:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Integer($crate::ast::Integer::new_dummy($n)),
		))
	};

	($n:literal $start:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Integer($crate::ast::Integer::new(
				$n,
				$start..$start + $n.to_string().len(),
			)),
		))
	};

	($n:literal $start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Integer($crate::ast::Integer::new(
				$n,
				$start..$end,
			)),
		))
	};
}

pub(crate) use n;

macro_rules! ident {
	($id:ident) => {
		Box::new($crate::ast::Expr::Ident(
			$crate::ast::Ident::new_dummy(stringify!($id)),
		))
	};

	($id:ident $start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::Ident(
			$crate::ast::Ident::new(stringify!($id), $start..$end),
		))
	};
}

pub(crate) use ident;

macro_rules! var {
	($id:ident) => {
		$crate::ast::Ident::new_dummy(stringify!($id))
	};

	($id:ident $start:literal.. $end:literal) => {
		$crate::ast::Ident::new(stringify!($id), $start..$end)
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
			$crate::ast::Literal::Nil($crate::ast::Nil::new_dummy()),
		))
	};

	($start:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Nil($crate::ast::Nil::new(
				$start..$start + 3,
			)),
		))
	};

	($start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Nil($crate::ast::Nil(start..end)),
		))
	};
}

pub(crate) use nil;

macro_rules! tru {
	() => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean::new_dummy(
				true,
			)),
		))
	};

	($start:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean::new(
				true,
				$start..$start + 4,
			)),
		))
	};

	($start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean::new(
				true,
				$start..$end,
			)),
		))
	};
}

pub(crate) use tru;

macro_rules! fal {
	() => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean::new_dummy(
				false,
			)),
		))
	};

	($start:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean::new(
				false,
				$start..$start + 5,
			)),
		))
	};

	($start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::Literal(
			$crate::ast::Literal::Boolean($crate::ast::Boolean::new(
				false,
				$start..$end,
			)),
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

macro_rules! call {
	($func:expr ; ( $($args:expr),* $(,)? )) => {
		Box::new(
			$crate::ast::Expr::Call(
				$func,
				$crate::ast::Arguments(vec![
					$($args),*
				])
			)
		)
	};
}

pub(crate) use call;

macro_rules! call_ident {
	($func:ident ( $($args:expr),* $(,)? )) => {
		$crate::ast::Expr::Call(
			Box::new($crate::ast::Expr::Ident(
				$crate::ast::Ident::new_dummy(
					stringify!($func),
				),
			)),
			$crate::ast::Arguments(vec![
				$($args),*
			])
		)
	};

	($func:ident $start:literal..$end:literal ( $($args:expr),* $(,)? )) => {
		$crate::ast::Expr::Call(
			Box::new($crate::ast::Expr::Ident(
				$crate::ast::Ident::new(
					stringify!($func),
					$start..$end
				),
			)),
			$crate::ast::Arguments(vec![
				$($args),*
			])
		)
	};
}

pub(crate) use call_ident;
