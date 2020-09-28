use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::nodes::statements::Statements;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Block<'a> {
	node: &'a SyntaxNode<'a>,
	statements: Statements<'a>,
	expression: Option<Expression<'a>>,
}

impl<'a> Block<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, statements: Statements<'a>, expression: Option<Expression<'a>>) -> Self {
		return Self {
			node,
			statements,
			expression,
		};
	}
}

impl<'a> Node<'a> for Block<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		engine.push_scope();
		execute!(engine, &self.statements);
		let reference = if let Some(expression) = &self.expression {
			execute!(engine, expression)
		} else {
			engine.undefined()
		};

		engine.pop_scope();
		return Ok(reference);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
