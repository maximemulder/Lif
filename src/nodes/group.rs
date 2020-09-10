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

impl Node for Group<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.execute(&self.expression);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
