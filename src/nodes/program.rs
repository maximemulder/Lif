use crate::nodes::Node;
use crate::nodes::statements::Statements;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		execute!(engine, &self.statements);
		return Ok(engine.new_undefined());
	}
}
