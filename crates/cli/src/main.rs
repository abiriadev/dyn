use std::{
	fmt::{self, Display, Formatter},
	fs::read_to_string,
	path::PathBuf,
};

use clap::{Parser, Subcommand, ValueEnum};
use dyn_core::{
	ArgumentValues, BuiltinFunction, FunctionValue, Interpreter, Value,
};
use lexer::SpannedLexer;
use maplit::hashmap;
use miette::Report;
use parser::ast::Ident;
use rustyline::DefaultEditor;

#[derive(Debug, Parser)]
struct Args {
	source_path: Option<PathBuf>,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
	Lexer {
		source_path: PathBuf,

		#[arg(short, long, default_value_t = LexerFormat::Ndjson)]
		format: LexerFormat,

		#[arg(short, long, default_value_t = String::from(","))]
		delimiter: String,
	},
}

#[derive(Debug, Clone, ValueEnum)]
enum LexerFormat {
	Ndjson,
	Csv,
}

impl Display for LexerFormat {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			LexerFormat::Ndjson => "ndjson",
			LexerFormat::Csv => "csv",
		})
	}
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

fn lexer_cmd(
	source_path: PathBuf,
	_format: LexerFormat,
	delimiter: String,
) -> anyhow::Result<()> {
	for t in SpannedLexer::new(&read_to_string(source_path)?) {
		let (l, tok, r) = t?;

		match _format {
			LexerFormat::Ndjson => println!(
				r#"{{"token":"{}","span":{{"start":{},"end":{}}}}}"#,
				tok, l, r
			),
			LexerFormat::Csv => println!("{l}{delimiter}{r}{delimiter}{tok}"),
		}
	}

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	if let Some(command) = args.command {
		match command {
			Commands::Lexer {
				source_path,
				format,
				delimiter,
			} => lexer_cmd(source_path, format, delimiter)?,
		}

		return Ok(())
	}

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
