use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Group<'a> {
	node: &'a SyntaxNode<'a>,
	expression: Expression<'a>,
}

impl<'a> Group<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression: Expression<'a>) -> Self {
		return Self {
			node,
			expression,
		};
	}
}

impl<'a> Node<'a> for Group<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return engine.execute(&self.expression);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
