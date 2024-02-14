use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Error)]
pub enum LexError {
	#[error("InvalidIdentifier")]
	InvalidIdentifier,

	#[default]
	#[error("InvalidToken")]
	InvalidToken,
}
