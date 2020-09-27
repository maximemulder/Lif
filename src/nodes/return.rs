use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };
use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;

pub struct Return<'a> {
	node: &'a SyntaxNode<'a>,
	expression: Option<Expression<'a>>}

impl<'a> Return<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression: Option<Expression<'a>>) -> Self {
		return Self {
			node,
			expression,
		};
	}
}

impl<'a> Node<'a> for Return<'a> {
	fn execute(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.control_new(Control::Return, &self.expression);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
