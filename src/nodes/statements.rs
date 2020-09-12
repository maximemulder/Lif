use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::statement::Statement;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statements<'a> {
	node: &'a SyntaxNode<'a>,
	statements: Vec<Statement<'a>>,
}

impl<'a> Statements<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, statements: Vec<Statement<'a>>) -> Self {
		return Self {
			node,
			statements,
		};
	}
}

impl Node for Statements<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		for statement in self.statements.iter() {
			execute!(engine, statement);
		}

		return Ok(engine.undefined());
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
