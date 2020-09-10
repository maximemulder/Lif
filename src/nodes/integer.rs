use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Integer<'a> {
	node: &'a SyntaxNode<'a>,
	integer: usize,
}

impl<'a> Integer<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, integer: usize) -> Self {
		return Self {
			node,
			integer,
		};
	}
}

impl Node for Integer<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return Ok(engine.new_integer(self.integer));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
