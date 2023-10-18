mod token;

#[cfg(test)] mod tests;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexError {
	InvalidIndentifier,

	#[default]
	InvalidToken,
}
