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
			statements: Statements::build(&node.children()[0]),
			expression: if node.children().len() == 2 {
				Some(Expression::build(&node.children()[1]))
			} else {
				None
			},
		};
	}
}

impl Node for Block {
	fn execute(&self) {
		self.statements.execute();
	}
}
