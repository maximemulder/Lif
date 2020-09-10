use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Continue<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	expression: Option<Expression<'a, 'b>>}

impl<'a, 'b> Continue<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, expression: Option<Expression<'a, 'b>>) -> Self {
		return Self {
			node,
			expression,
		};
	}
}

impl Node for Continue<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.new_control(Control::Continue, &self.expression);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
