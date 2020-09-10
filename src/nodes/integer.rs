use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Integer<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	integer: usize,
}

impl<'a, 'b> Integer<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, integer: usize) -> Self {
		return Self {
			node,
			integer,
		};
	}
}

impl Node for Integer<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return Ok(engine.new_integer(self.integer));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
