use crate::nodes::Node;
use crate::nodes::statement::Statement;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

pub struct Statements {
	statements: Vec<Statement>,
}

impl Statements {
	pub fn new(statements: Vec<Statement>) -> Self {
		return Self {
			statements,
		};
	}
}

impl Node for Statements {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		for statement in self.statements.iter() {
			execute!(engine, statement);
		}

		return Ok(engine.new_undefined());
	}
}
