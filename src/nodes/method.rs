use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;

pub struct Method<'a> {
	node: &'a SyntaxNode<'a>,
	expression: Expression<'a>,
	member:     &'a str,
	expressions: Vec<Expression<'a>>,
}

impl<'a> Method<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression: Expression<'a>, member: &'a str, expressions: Vec<Expression<'a>>) -> Self {
		return Self {
			node,
			expression,
			member,
			expressions,
		};
	}
}

impl<'a> Node<'a> for Method<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let this = execute!(engine, &self.expression).read()?;
		if let Some(method) = this.get_method(engine, self.member) {
			let mut arguments = Vec::new();
			arguments.push(this);
			for argument in self.expressions.iter() {
				arguments.push(execute!(engine, argument).read()?);
			}

			return engine.call(method, arguments);
		}

		return Err(Error::new_runtime("Method does not exist."));
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
