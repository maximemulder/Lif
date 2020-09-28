use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Sequence<'a> {
	node: &'a SyntaxNode<'a>,
	expression:  Expression<'a>,
	open:        &'a str,
	expressions: Vec<Expression<'a>>,
	close:       &'a str,
}

impl<'a> Sequence<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression:  Expression<'a>, open: &'a str, expressions: Vec<Expression<'a>>, close: &'a str) -> Self {
		return Self {
			node,
			expression,
			open,
			expressions,
			close,
		};
	}
}

impl<'a> Node<'a> for Sequence<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let value = execute!(engine, &self.expression).read()?;
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(execute!(engine, argument).read()?);
		}

		let mut name = String::new();
		name.push_str(&self.open);
		name.push_str(&self.close);
		return engine.call_method(value, &name, arguments);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
