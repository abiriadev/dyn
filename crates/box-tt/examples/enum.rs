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

fn main() {
	Error::timeout_error_box(123);
	Error::rgb_error_box(1, 2, 3);
	Error::file_write_error_box("aa".to_owned(), PathBuf::from("./asdf"));
	Error::path_does_not_exist_error_box(PathBuf::from("./notexist.txt"));
}
