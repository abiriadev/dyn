use std::fmt::{self, Display, Formatter};

use dyn_lexer::LexError;
use miette::{Diagnostic, LabeledSpan};
use parser::ast::{BinExprKind, Ident, UnaryExprKind};
use span::{HasSpan, Span};
use thiserror::Error;

use crate::Value;

#[derive(Debug, PartialEq, Error)]
#[error("InterpreterError")]
pub enum InterpreterError {
	ParseError(#[from] ParseError),
	RuntimeError(#[from] RuntimeError),
}

impl Diagnostic for InterpreterError {
	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		match self {
			Self::ParseError(t) => t.labels(),
			Self::RuntimeError(t) => t.labels(),
		}
	}
}

#[derive(Debug, PartialEq, Error)]
#[error("ParseError")]
pub struct ParseError(#[from] pub parser::ParseError);

impl Diagnostic for ParseError {
	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		use parser::ParseError;

		match &self.0 {
			ParseError::InvalidToken { .. } => unreachable!(),
			ParseError::UnrecognizedEof { location, expected } => {
				Some(Box::new(
					[LabeledSpan::at(
						*location..location + 1,
						format!(
							"Expected one of {} but found EOF",
							display_vec(expected)
						),
					)]
					.into_iter(),
				))
			},
			ParseError::UnrecognizedToken { token, expected } => {
				Some(Box::new(
					[LabeledSpan::at(
						token.0..token.2,
						format!(
							"Expected one of {} but found {:?}",
							display_vec(expected),
							token.1
						),
					)]
					.into_iter(),
				))
			},
			ParseError::ExtraToken { .. } => unimplemented!(),
			ParseError::User { error } => {
				let span = error.span();

				let m = match error.get() {
					LexError::InvalidIdentifier => "Invalid Identifier",
					LexError::InvalidToken => "Invalid Token",
				};

				Some(Box::new(
					[LabeledSpan::at(span, m)].into_iter(),
				))
			},
		}
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Error)]
pub enum RuntimeError {
	#[error("Reference Error")]
	ReferenceError(#[from] ReferenceError),

	#[error("Assignment to immutable variable")]
	AssignmentToImmutableVariable,

	#[error("Already declared")]
	AlreadyDeclared,

	#[error("Type Error")]
	TypeError(#[from] TypeError),
}

impl Diagnostic for RuntimeError {
	fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
		match self {
			Self::TypeError(t) => Some(t),
			Self::ReferenceError(t) => Some(t),
			_ => None,
		}
	}

	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		match self {
			RuntimeError::TypeError(t) => t.labels(),
			Self::ReferenceError(t) => t.labels(),
			_ => None,
		}
	}
}

#[derive(Debug, PartialEq, Error)]
pub enum TypeError {
	UnaryOp {
		op: UnaryExprKind,
		expr: Value,
		expr_span: Span,
	},
	BinOp {
		op: BinExprKind,
		lhs: Value,
		lhs_span: Span,
		rhs: Value,
		rhs_span: Span,
	},
}

impl Display for TypeError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let m = match self {
			TypeError::UnaryOp { op, expr, .. } => match op {
				UnaryExprKind::Minus => {
					format!("{} is not a number", expr.to_debug())
				},
				_ => format!(
					"cannot apply unary operator `{}` to type `{}`",
					op.as_ref(),
					expr.get_type().type_name()
				),
			},
			TypeError::BinOp { op, lhs, rhs, .. } => match op {
				BinExprKind::Add => format!(
					"cannot add `{}` to `{}`",
					rhs.get_type().type_name(),
					lhs.get_type().type_name()
				),
				BinExprKind::Sub => format!(
					"cannot subtract `{}` from `{}`",
					rhs.get_type().type_name(),
					lhs.get_type().type_name()
				),
				BinExprKind::Mul => format!(
					"cannot multiply `{}` by `{}`",
					rhs.get_type().type_name(),
					lhs.get_type().type_name()
				),
				BinExprKind::Div => format!(
					"cannot divide `{}` by `{}`",
					rhs.get_type().type_name(),
					lhs.get_type().type_name()
				),
				BinExprKind::Mod => format!(
					"cannot calculate the remainder of `{}` divided by `{}`",
					rhs.get_type().type_name(),
					lhs.get_type().type_name()
				),
				op => {
					format!(
						"`{}` cannot be applied to `{}` and `{}`",
						op.as_ref(),
						rhs.get_type().type_name(),
						lhs.get_type().type_name()
					)
				},
			},
		};

		write!(f, "{m}")
	}
}

impl Diagnostic for TypeError {
	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		match self {
			TypeError::UnaryOp {
				expr, expr_span, ..
			} => Some(Box::new(
				[value_to_message(*expr_span, expr.clone())].into_iter(),
			)),
			TypeError::BinOp {
				lhs,
				lhs_span,
				rhs,
				rhs_span,
				..
			} => Some(Box::new(
				[
					value_to_message(*lhs_span, lhs.clone()),
					value_to_message(*rhs_span, rhs.clone()),
				]
				.into_iter(),
			)),
		}
	}
}

#[derive(Debug, PartialEq, Error)]
pub enum ReferenceError {
	#[error("UndefinedIdentifier")]
	UndefinedIdentifier { ident: Ident },
}

impl Diagnostic for ReferenceError {
	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		match self {
			ReferenceError::UndefinedIdentifier { ident } => Some(Box::new(
				[LabeledSpan::at(
					ident.span(),
					format!(
						"can't find `{}` in this scope",
						ident.symbol()
					),
				)]
				.into_iter(),
			)),
		}
	}
}

fn value_to_message(span: Span, v: Value) -> LabeledSpan {
	LabeledSpan::at(
		span,
		format!(
			"{} is of type {}",
			v.to_debug(),
			v.get_type().type_name()
		),
	)
}

fn display_vec<T>(v: &[T]) -> String
where
	T: Display,
{
	v.iter()
		.map(|e| e.to_string())
		.collect::<Vec<_>>()
		.join(", ")
}
