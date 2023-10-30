use miette::{Diagnostic, LabeledSpan, SourceSpan};
use parser::{LexError, ParseError, Token};
use thiserror::Error;

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
	TypeError { lhs: LabeledSpan, rhs: LabeledSpan },
}

impl Diagnostic for RuntimeError {
	fn labels(
		&self,
	) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
		match self {
			RuntimeError::TypeError { lhs, rhs } => Some(Box::new(
				[lhs.clone(), rhs.clone()].into_iter(),
			)),
			_ => None,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
}
