use std::path::PathBuf;

use box_tt::BoxNew;

#[derive(BoxNew)]
enum Error {
	UnknownError,
	TimeoutError(u16),
	PathDoesNotExistError(Box<PathBuf>),
	FileWriteError(String, Box<PathBuf>),
	RgbError(Box<u8>, Box<u8>, Box<u8>),
}

fn main() { todo!() }
