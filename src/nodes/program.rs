use crate::nodes::Node;
use crate::nodes::statements::Statements;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Program {
	statements: Statements,
}

impl Program {
	pub fn new(statements: Statements) -> Self {
		return Self {
			statements,
		};
	}
}

impl Node for Program {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		execute!(engine, &self.statements);
		return Ok(engine.new_undefined());
	}
}
