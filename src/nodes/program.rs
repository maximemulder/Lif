use crate::nodes::Node;
use crate::nodes::statements::Statements;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		execute!(engine, &self.statements);
		return engine.new_undefined();
	}
}
