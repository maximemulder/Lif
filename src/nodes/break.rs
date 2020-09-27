use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Break<'a> {
	node: &'a SyntaxNode<'a>,
	expression: Option<Expression<'a>>,
}

impl<'a> Break<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression: Option<Expression<'a>>) -> Self {
		return Self {
			node,
			expression,
		};
	}
}

impl<'a> Node<'a> for Break<'a> {
	fn execute(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.control_new(Control::Break, &self.expression);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
