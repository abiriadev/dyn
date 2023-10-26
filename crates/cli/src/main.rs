use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use dyn_core::Interpreter;

#[derive(Debug, Parser)]
struct Args {
	source_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let source = read_to_string(args.source_path)?;

	let mut intpr = Interpreter::init();
	let res = intpr.run(&source);

	println!("final result: {:?}", res);
	Ok(())
}
