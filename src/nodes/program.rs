use crate::nodes::statements::Statements;
use crate::runtime::Engine;
use super::{ Node, Product };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		self.statements.execute(engine);
		return Product::new(engine.new_undefined());
	}
}
