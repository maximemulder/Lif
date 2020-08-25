use crate::nodes::statements::Statements;
use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		self.statements.execute(engine);
		return Product::new(Reference::new_undefined());
	}
}
