#![allow(unused)]

macro_rules! n {
	($n:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Integer(
				$crate::ast::Integer::new_dummy($n),
			)),
			span::Span::DUMMY_SPAN,
		))
	};

	($n:literal $start:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Integer(
				$crate::ast::Integer::new(
					$n,
					$start..$start + $n.to_string().len(),
				),
			)),
			$start..$start + $n.to_string().len(),
		))
	};

	($n:literal $start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Integer(
				$crate::ast::Integer::new($n, $start..$end),
			)),
			$crate::ast::Integer::new($n, $start..$end),
		))
	};
}

pub(crate) use n;

macro_rules! ident {
	($id:ident) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Ident($crate::ast::Ident::new_dummy(
				stringify!($id),
			)),
			span::Span::DUMMY_SPAN,
		))
	};

	($id:ident $start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Ident($crate::ast::Ident::new(
				stringify!($id),
				$start..$end,
			)),
			$start..$end,
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
			$crate::ast::Literal::String($crate::ast::StringT::new_dummy($str)),
		))
	};

	($str:literal $start:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::String(
				$crate::ast::StringT::new(
					$str,
					$start..$start + $str.len() + 2,
				),
			)),
			$start..$start + $str.len() + 2,
		))
	};

	($str:literal $start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::String(
				$crate::ast::Literal::String($crate::ast::StringT::new(
					$str,
					$start..$end,
				)),
			)),
			$start..$end,
		))
	};
}
pub(crate) use str;

macro_rules! arr {
	[$($ele:expr),* $(,)?] => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Array(
				$crate::ast::Array::new_dummy(
					vec![$($ele),*]
				)
			),
			span::Span::DUMMY_SPAN,
		))
	};

	[$($ele:expr),* $(,)? ; $start:literal] => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Array(
				$crate::ast::Array::new(
					vec![$($ele),*],
					$start..$start + 2
				)
			),
			$start..$start + 2
		))
	};

	[$($ele:expr),* $(,)? ; $start:literal .. $end:literal] => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Array(
				$crate::ast::Array::new(
					vec![$($ele),*],
					$start..$end
				)
			),
			$start..$end
		))
	};
}
pub(crate) use arr;

macro_rules! nil {
	() => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Nil(
				$crate::ast::Nil::new_dummy(),
			)),
			span::Span::DUMMY_SPAN,
		))
	};

	($start:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Nil(
				$crate::ast::Nil::new($start..$start + 3),
			)),
			$start..$start + 3,
		))
	};

	($start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Nil(
				$crate::ast::Nil($start..$end),
			)),
			$start..$end,
		))
	};
}

pub(crate) use nil;

macro_rules! tru {
	() => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Boolean(
				$crate::ast::Boolean::new_dummy(true),
			)),
			span::Span::DUMMY_SPAN,
		))
	};

	($start:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Boolean(
				$crate::ast::Boolean::new(true, $start..$start + 4),
			)),
			$start..$start + 4,
		))
	};

	($start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Boolean(
				$crate::ast::Boolean::new(true, $start..$end),
			)),
			$start..$end,
		))
	};
}

pub(crate) use tru;

macro_rules! fal {
	() => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Boolean(
				$crate::ast::Boolean::new_dummy(false),
			)),
			span::Span::DUMMY_SPAN,
		))
	};

	($start:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Boolean(
				$crate::ast::Boolean::new(false, $start..$start + 5),
			)),
			$start..$start + 5,
		))
	};

	($start:literal.. $end:literal) => {
		Box::new($crate::ast::Expr::new(
			$crate::ast::ExprKind::Literal($crate::ast::Literal::Boolean(
				$crate::ast::Boolean::new(false, $start..$end),
			)),
			$start..$end,
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
	($func:expr ; $start:literal .. $end:literal ( $($args:expr),* $(,)? )) => {
		Box::new(
			$crate::ast::Expr::new(
				$crate::ast::ExprKind::Call(
					$func,
					$crate::ast::Arguments(vec![
						$($args),*
					])
				),
				$start..$end
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
		$crate::ast::Expr::new(
			$crate::ast::ExprKind::Call(
				Box::new($crate::ast::Expr::new(
					$crate::ast::ExprKind::Ident($crate::ast::Ident::new(
						stringify!($func),
						$start..$start + stringify!($func).len(),
					)),
					$start..$start + stringify!($func).len(),
				)),
				$crate::ast::Arguments(vec![
					$($args),*
				])
			),
			$start..$end,
		)
	};
}

pub(crate) use call_ident;
