use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Generic<'a> {
	node: &'a SyntaxNode<'a>,
	parameters: Vec<&'a str>,
	exe: Box<dyn Node<'a> + 'a>,
}

impl<'a> Generic<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, parameters: Vec<&'a str>, exe: Box<dyn Node<'a> + 'a>) -> Self {
		return Self {
			node,
			parameters,
			exe,
		};
	}
}

impl<'a> Node<'a> for Generic<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return Ok(engine.new_generic(&self.parameters, self.exe.as_ref()));
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
