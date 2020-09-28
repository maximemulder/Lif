use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Expression<'a> {
	node: &'a SyntaxNode<'a>,
	exe: Box<dyn Node<'a> + 'a>,
}

impl<'a> Expression<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, exe: Box<dyn Node<'a> + 'a>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl<'a> Node<'a> for Expression<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		engine.collect();
		return engine.execute(self.exe.as_ref());
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
