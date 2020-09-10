use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Identifier<'a> {
	node: &'a SyntaxNode<'a>,
	identifier: Box<str>,
}

impl<'a> Identifier<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, identifier: Box<str>) -> Self {
		return Self {
			node,
			identifier,
		};
	}
}

impl Node for Identifier<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.get_variable(&self.identifier);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
