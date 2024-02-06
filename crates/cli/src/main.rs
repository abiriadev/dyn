use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use dyn_core::{
	ArgumentValues, BuiltinFunction, FunctionValue, Interpreter, Value,
};
use maplit::hashmap;
use miette::Report;
use parser::ast::Ident;
use rustyline::DefaultEditor;

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
		Ident::new_dummy("print") => Value::Function(
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
		let mut rl = DefaultEditor::new().unwrap();

		loop {
			let readline = rl.readline("> ");
			match readline {
				Ok(line) => {
					rl.add_history_entry(line.as_str())
						.unwrap();

					let res = intpr.run(&line);

					match res {
						Ok(v) => println!("{}", v.to_debug()),
						Err(e) => {
							let rep: Report = e.into();
							let rep = rep.with_source_code(line);

							println!("{rep:?}");
						},
					};
				},
				Err(_) => {
					println!("exit!");
					break;
				},
			}
		}
	}

	Ok(())
}
