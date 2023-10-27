use parser::{LexError, ParseError, Token};

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
	ParseError(ParseError<usize, Token, LexError>),
	RuntimeError(RuntimeError),
}

impl From<RuntimeError> for InterpreterError {
	fn from(value: RuntimeError) -> Self { Self::RuntimeError(value) }
}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
	ReferenceError(ReferenceError),
	AssignmentToImmutableVariable,
	AlreadyDeclared,
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
}
