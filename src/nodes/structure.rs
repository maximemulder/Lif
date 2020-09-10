use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Structure<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	exe: Box<dyn Node + 'b>,
}

impl<'a, 'b> Structure<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, exe: Box<dyn Node + 'b>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl Node for Structure<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.execute(self.exe.as_ref());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
