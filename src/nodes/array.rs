use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Array<'a> {
	node: &'a SyntaxNode<'a>,
	expressions: Vec<Expression<'a>>,
}

impl<'a> Array<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expressions: Vec<Expression<'a>>) -> Self {
		return Self {
			node,
			expressions,
		};
	}
}

impl<'a> Node<'a> for Array<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let mut references = Vec::new();
		for expression in self.expressions.iter() {
			let value = execute!(engine, expression).read()?;
			references.push(engine.new_reference(value));
		}

		return Ok(engine.new_array(references));
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
