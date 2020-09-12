use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statement<'a> {
	node: &'a SyntaxNode<'a>,
	exe: Box<dyn Node + 'a>,
}

impl<'a> Statement<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, exe: Box<dyn Node + 'a>) -> Self {
		return Self {
			node,
			exe,
		};
	}
}

impl Node for Statement<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		execute!(engine, self.exe.as_ref());
		return Ok(engine.undefined());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
