use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct False<'a> {
	node: &'a SyntaxNode<'a>,
}

impl<'a> False<'a> {
	pub fn new(node: &'a SyntaxNode<'a>) -> Self {
		return Self {
			node,
		};
	}
}

impl<'a> Node<'a> for False<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return Ok(engine.new_boolean(false));
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
