use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Group<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	expression: Expression<'a, 'b>,
}

impl<'a, 'b> Group<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, expression: Expression<'a, 'b>) -> Self {
		return Self {
			node,
			expression,
		};
	}
}

impl Node for Group<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.execute(&self.expression);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
