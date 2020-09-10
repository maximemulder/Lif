use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Expression<'a> {
	node: &'a SyntaxNode<'a>,
	exe: Box<dyn Node + 'a>,
}

impl<'a> Expression<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, exe: Box<dyn Node + 'a>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl Node for Expression<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		engine.collect();
		return engine.execute(self.exe.as_ref());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
