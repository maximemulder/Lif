use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Identifier<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	identifier: Box<str>,
}

impl<'a, 'b> Identifier<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, identifier: Box<str>) -> Self {
		return Self {
			node,
			identifier,
		};
	}
}

impl Node for Identifier<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.get_variable(&self.identifier);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
