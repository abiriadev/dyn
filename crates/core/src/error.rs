use std::fmt::{self, Display, Formatter};

use miette::{Diagnostic, LabeledSpan};
use parser::{ast::BinExprKind, LexError, Token};
use span::Span;
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
pub struct ParseError(pub parser::ParseError<usize, Token, LexError>);

impl Diagnostic for ParseError {
	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		use parser::ParseError;

		match &self.0 {
			ParseError::InvalidToken { location } => todo!(),
			ParseError::UnrecognizedEof { location, expected } => todo!(),
			ParseError::UnrecognizedToken { token, expected } => todo!(),
			ParseError::ExtraToken { token } => todo!(),
			ParseError::User { error } => {
				let m = match error {
					LexError::InvalidIdentifier => "Invalid Identifier",
					LexError::InvalidToken => "Invalid Token",
				};

				Some(Box::new(
					[LabeledSpan::at(Span::DUMMY_SPAN, m)].into_iter(),
				))
			},
		}
	}
}

#[derive(Debug, PartialEq, Error)]
pub enum RuntimeError {
	#[error("Reference Error")]
	ReferenceError(ReferenceError),

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
			_ => None,
		}
	}

	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		match self {
			RuntimeError::TypeError(t) => t.labels(),
			_ => None,
		}
	}
}

#[derive(Debug, PartialEq, Error)]
pub enum TypeError {
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
		Some(Box::new(match self {
			TypeError::BinOp {
				lhs,
				lhs_span,
				rhs,
				rhs_span,
				..
			} => [
				value_to_message(*lhs_span, lhs.clone()),
				value_to_message(*rhs_span, rhs.clone()),
			]
			.into_iter(),
		}))
	}
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
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
