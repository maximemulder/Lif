use crate::runtime::{ Engine, Reference };
use super::statement::Statement;
use super::{ Node, SyntaxNode };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		for statement in self.statements.iter() {
			statement.execute(engine);
		}

		return engine.new_undefined();
	}
}
