use std::{
	fmt::{self, Display, Formatter},
	fs::read_to_string,
	path::PathBuf,
};

use clap::{Parser, Subcommand, ValueEnum};
use dyn_core::{
	ArgumentValues, BuiltinFunction, FunctionValue, Interpreter, Value,
};
use lexer::{lexer::LexerConfig, SpannedLexer, SpannedToken};
use maplit::hashmap;
use miette::Report;
use parser::ast::Ident;
use rustyline::DefaultEditor;

#[derive(Debug, Parser)]
struct Args {
	source_path: Option<PathBuf>,

	#[arg(short, long, default_value_t = false)]
	print_last_expression: bool,

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

		#[arg(short = 'w', long, default_value_t = false)]
		ignore_whitespace: bool,

		#[arg(short = 'c', long, default_value_t = false)]
		ignore_comments: bool,

		#[arg(short, long, default_value_t = false)]
		asi: bool,
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
	lexer_cfg: LexerConfig,
) -> anyhow::Result<()> {
	for SpannedToken { token, span } in
		SpannedLexer::new(&read_to_string(source_path)?, lexer_cfg)
	{
		let (l, r) = span.into();
		let tok = token?;

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
				ignore_whitespace,
				ignore_comments,
				asi,
			} => lexer_cmd(
				source_path,
				format,
				delimiter,
				LexerConfig {
					ignore_whitespace,
					ignore_comments,
					asi,
				},
			)?,
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
		} else if args.print_last_expression {
			match res {
				Ok(v) => println!("{v}"),
				Err(_) => unimplemented!(),
			}
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
