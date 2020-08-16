use crate::runtime::Engine;
use super::statement::Statement;
use super::{ Node, Product };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		for statement in self.statements.iter() {
			value!(statement.execute(engine));
		}

		return Product::new(engine.new_undefined());
	}
}
