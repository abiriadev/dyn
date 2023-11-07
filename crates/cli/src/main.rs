use std::{fs::read_to_string, io::stdin, path::PathBuf};

use clap::Parser;
use dyn_core::{
	ArgumentValues, BuiltinFunction, FunctionValue, Interpreter, ResolvedIdent,
	Value,
};
use maplit::hashmap;
use miette::Report;

#[derive(Debug, Parser)]
struct Args {
	source_path: Option<PathBuf>,
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

	let mut intpr = Interpreter::init_with_builtins(hashmap! {
		ResolvedIdent::new("print") => Value::Function(
			FunctionValue::Builtin(Printer::new())
		),
	})
	.unwrap();

	if let Some(p) = args.source_path {
		let source = read_to_string(p).unwrap();

		let res = intpr.run(&source);

		if let Err(diag) = res {
			let rep: Report = diag.into();
			let rep = rep.with_source_code(source);

			println!("{rep:?}");
		} else {
			println!("final result: {:?}", res);
		}
	} else {
		for l in stdin().lines() {
			let l = l.unwrap();

			let res = intpr.run(&l);

			if let Err(diag) = res {
				let rep: Report = diag.into();
				let rep = rep.with_source_code(l);

				println!("{rep:?}");
			} else {
				println!("final result: {:?}", res);
			}
		}
	}

	Ok(())
}
