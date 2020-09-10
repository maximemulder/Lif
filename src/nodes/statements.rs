use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::statement::Statement;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statements<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	statements: Vec<Statement<'a, 'b>>,
}

impl<'a, 'b> Statements<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, statements: Vec<Statement<'a, 'b>>) -> Self {
		return Self {
			node,
			statements,
		};
	}
}

impl Node for Statements<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		for statement in self.statements.iter() {
			execute!(engine, statement);
		}

		return Ok(engine.new_undefined());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
