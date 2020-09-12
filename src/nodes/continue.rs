use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Continue<'a> {
	node: &'a SyntaxNode<'a>,
	expression: Option<Expression<'a>>}

impl<'a> Continue<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression: Option<Expression<'a>>) -> Self {
		return Self {
			node,
			expression,
		};
	}
}

impl Node for Continue<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.control_new(Control::Continue, &self.expression);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
