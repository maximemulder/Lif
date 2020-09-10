use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::statements::Statements;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Program<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	statements: Statements<'a, 'b>,
}

impl<'a, 'b> Program<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, statements: Statements<'a, 'b>) -> Self {
		return Self {
			node,
			statements,
		};
	}
}

impl Node for Program<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		execute!(engine, &self.statements);
		return Ok(engine.new_undefined());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
