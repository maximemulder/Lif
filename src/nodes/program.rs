use crate::nodes::statements::Statements;
use crate::runtime::Engine;
use super::{ Node, SyntaxNode, Product };

pub struct Program {
	statements: Statements,
}

impl Program {
	pub fn build(node: &SyntaxNode) -> Program {
		return Program {
			statements: Statements::build(&node.children()[0]),
		};
	}
}

impl Node for Program {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		self.statements.execute(engine);
		return Product::new(engine.new_undefined());
	}
}
