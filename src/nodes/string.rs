use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct String<'a> {
	node: &'a SyntaxNode<'a>,
	string: Box<str>,
}

impl<'a> String<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, string: Box<str>) -> Self {
		return Self {
			node,
			string,
		};
	}
}

impl Node for String<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return Ok(engine.new_string(self.string.to_string()));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
