use miette::{Diagnostic, LabeledSpan, SourceSpan};
use parser::{LexError, ParseError, Token};
use span::Span;
use thiserror::Error;

use crate::Value;

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
	ParseError(ParseError<usize, Token, LexError>),
	RuntimeError(RuntimeError),
}

impl From<RuntimeError> for InterpreterError {
	fn from(value: RuntimeError) -> Self { Self::RuntimeError(value) }
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
	#[error("type does not match")]
	BinOp { lhs: LabeledSpan, rhs: LabeledSpan },
}

impl Diagnostic for TypeError {
	fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
		Some(Box::new(match self {
			TypeError::BinOp { lhs, rhs } =>
				[lhs.clone(), rhs.clone()].into_iter(),
		}))
	}
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
}

pub fn value_to_message(span: Span, v: Value) -> LabeledSpan {
	LabeledSpan::at(
		span,
		format!(
			"{} is of type {}",
			v.to_debug(),
			v.get_type().type_name()
		),
	)
}
