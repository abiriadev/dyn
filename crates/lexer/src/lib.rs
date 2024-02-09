use std::marker::PhantomData;

use thiserror::Error;

mod lexer;
mod token;

use span::Spanned;
pub use token::{QuotedString, Token};

// #[cfg(test)] mod tests;

#[derive(Default, Debug, Clone, PartialEq, Error)]
pub enum LexError {
	#[error("InvalidIdentifier")]
	InvalidIdentifier,

	#[default]
	#[error("InvalidToken")]
	InvalidToken,
}
