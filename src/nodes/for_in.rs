use super::{ Engine, Node, SyntaxNode };
use super::expression::Expression;
use super::r#do::Do;
use super::token::Token;

pub struct ForIn {
	identifier: Box<str>,
	expression: Expression,
	body:       Expression,
}

impl ForIn {
	pub fn build(node: &SyntaxNode) -> ForIn {
		return ForIn {
			identifier: Token::build(&node.children()[1]),
			expression: Expression::build(&node.children()[3]),
			body:       Do::build(&node.children()[4]),
		};
	}
}

impl Node for ForIn {
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		return None;
	}
}
