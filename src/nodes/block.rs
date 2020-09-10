use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::nodes::statements::Statements;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Block<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	statements: Statements<'a, 'b>,
	expression: Option<Expression<'a, 'b>>,
}

impl<'a, 'b> Block<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, statements: Statements<'a, 'b>, expression: Option<Expression<'a, 'b>>) -> Self {
		return Self {
			node,
			statements,
			expression,
		};
	}
}

impl Node for Block<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		engine.push_scope();
		execute!(engine, &self.statements);
		let reference = if let Some(expression) = &self.expression {
			execute!(engine, expression)
		} else {
			engine.new_undefined()
		};

		engine.pop_scope();
		return Ok(reference);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
