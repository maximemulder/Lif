use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct String<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	string: Box<str>,
}

impl<'a, 'b> String<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, string: Box<str>) -> Self {
		return Self {
			node,
			string,
		};
	}
}

impl Node for String<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return Ok(engine.new_string(self.string.to_string()));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
