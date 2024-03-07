use serde::Serialize;
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Error, Serialize)]
pub enum LexError {
	#[error("InvalidIdentifier")]
	InvalidIdentifier,

	#[default]
	#[error("InvalidToken")]
	InvalidToken,
}
