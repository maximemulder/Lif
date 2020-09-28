use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Identifier<'a> {
	node: &'a SyntaxNode<'a>,
	identifier: &'a str,
}

impl<'a> Identifier<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, identifier: &'a str) -> Self {
		return Self {
			node,
			identifier,
		};
	}
}

impl<'a> Node<'a> for Identifier<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return engine.get_variable(&self.identifier);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
