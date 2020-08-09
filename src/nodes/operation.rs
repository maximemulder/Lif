use super::expression::Expression;
use super::token::Token;
use super::{ Engine, Node, SyntaxNode };

pub struct Operation {
	left:     Expression,
	right:    Expression,
	operator: Box<str>,
}

impl Operation {
	pub fn build(node: &SyntaxNode) -> Operation {
		return Operation {
			left:     Expression::build(&node.children()[0]),
			operator: Token::build(&node.children()[1]),
			right:    Expression::build(&node.children()[2]),
		};
	}
}

impl Node for Operation {
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		self.left.execute(engine);
		self.right.execute(engine);
		return None;
	}
}
