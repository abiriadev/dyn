use std::io::{Read, Write};

pub struct CompilerConfig {
	std: (),
}

impl CompilerConfig {}

pub struct Compiler {}

impl Compiler {
	pub fn new(config: CompilerConfig) -> Self {
		todo!()
	}

	pub fn session<S, I, O, E>(
		&self,
		config: SessionConfig<S, I, O, E>,
	) -> Session
	where
		S: Read,
		I: Read,
		O: Write,
		E: Write,
	{
		todo!()
	}
}

pub struct SessionConfig<S, I, O, E>
where
	S: Read,
	I: Read,
	O: Write,
	E: Write,
{
	source: S,
	stdin: I,
	stdout: O,
	stderr: E,
}

pub struct Session {}
