use miette::{Diagnostic, SourceSpan};
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

#[derive(Debug, PartialEq, Error, Diagnostic)]
pub enum RuntimeError {
	#[error("Reference Error")]
	ReferenceError(ReferenceError),

	#[error("Assignment to immutable variable")]
	AssignmentToImmutableVariable,

	#[error("Already declared")]
	AlreadyDeclared,

	#[error("Type Error")]
	TypeError {
		#[label("location")]
		location: SourceSpan,
	},
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
}
