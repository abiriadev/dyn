use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use clap::Parser;
use dyn_core::{
	ArgumentValues, BuiltinFunction, FunctionValue, Interpreter, ResolvedIdent,
	Value,
};
use maplit::hashmap;
use parser::ast::Ident;

#[derive(Debug, Parser)]
struct Args {
	source_path: PathBuf,
}

#[derive(Debug, Clone)]
struct Printer;

impl Printer {
	fn new() -> Box<Self> { Box::new(Self) }
}

impl BuiltinFunction for Printer {
	fn call(&mut self, args: ArgumentValues) -> Value {
		let msg = args.0.into_iter().nth(0).unwrap();
		println!("{}", msg.clone());
		msg
	}
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let source = read_to_string(args.source_path)?;

	let mut intpr = Interpreter::init_with_builtins(hashmap! {
		ResolvedIdent::new("print") => Value::Function(
			FunctionValue::Builtin(Printer::new())
		),
	})
	.unwrap();
	let res = intpr.run(&source);

	println!("final result: {:?}", res);
	Ok(())
}
