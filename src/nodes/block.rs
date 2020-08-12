use crate::runtime::{ Engine, Reference };
use super::expression::Expression;
use super::statements::Statements;
use super::{ Node, SyntaxNode };

pub struct Block {
	statements: Statements,
	expression: Option<Expression>,
}

impl Block {
	pub fn build(node: &SyntaxNode) -> Block {
		return Block {
			statements: Statements::build(&node.children()[1]),
			expression: if node.children().len() == 4 {
				Some(Expression::build(&node.children()[2]))
			} else {
				None
			},
		};
	}
}

impl Node for Block {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		engine.push_scope();
		self.statements.execute(engine);
		let value = if let Some(expression) = &self.expression {
			expression.execute(engine)
		} else {
			engine.new_undefined()
		};

		engine.pop_scope();
		return value;
	}
}
