use crate::runtime::Engine;
use super::statement::Statement;
use super::{ Node, SyntaxNode, Product };

pub struct Statements {
	statements: Vec<Statement>,
}

impl Statements {
	pub fn build(node: &SyntaxNode) -> Statements {
		let mut statements = Vec::new();
		for child in node.children() {
			statements.push(Statement::build(child));
		}

		return Statements {
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
