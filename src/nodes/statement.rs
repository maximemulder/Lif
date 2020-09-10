use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statement<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	exe: Box<dyn Node + 'b>,
}

impl<'a, 'b> Statement<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, exe: Box<dyn Node + 'b>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl Node for Statement<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		execute!(engine, self.exe.as_ref());
		return Ok(engine.new_undefined());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
