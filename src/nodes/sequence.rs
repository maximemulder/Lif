use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Sequence<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	expression:  Expression<'a, 'b>,
	open:        Box<str>,
	expressions: Vec<Expression<'a, 'b>>,
	close:       Box<str>,
}

impl<'a, 'b> Sequence<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, expression:  Expression<'a, 'b>, open: Box<str>, expressions: Vec<Expression<'a, 'b>>, close: Box<str>) -> Self {
		return Self {
			node,
			expression,
			open,
			expressions,
			close,
		};
	}
}

impl Node for Sequence<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let value = execute!(engine, &self.expression).read()?;
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(execute!(engine, argument).read()?);
		}

		return engine.call(value, arguments);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
