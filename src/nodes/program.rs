use crate::nodes::statements::Statements;
use crate::runtime::{ Engine, Reference };
use super::{ Node, SyntaxNode };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		self.statements.execute(engine);
		return engine.new_undefined();
	}
}
