pub mod ast;
mod lexer;
mod macros;
mod parser;

pub use parser::parse;
