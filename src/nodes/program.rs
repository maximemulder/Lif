use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::statements::Statements;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Program<'a> {
	node: &'a SyntaxNode<'a>,
	statements: Statements<'a>,
}

impl<'a> Program<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, statements: Statements<'a>) -> Self {
		return Self {
			node,
			statements,
		};
	}
}

impl<'a> Node<'a> for Program<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		execute!(engine, &self.statements);
		return Ok(engine.undefined());
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
