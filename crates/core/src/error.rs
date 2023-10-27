use parser::{LexError, ParseError, Token};

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
	ParseError(ParseError<usize, Token, LexError>),
	RuntimeError(RuntimeError),
}

#[derive(Debug, PartialEq)]
pub enum ReferenceError {
	UndefinedIdentifier,
}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
	ReferenceError(ReferenceError),
	AssignmentToImmutableVariable,
	AlreadyDeclared,
}
