use super::expression::Expression;
use super::{ Node, SyntaxNode };

pub struct Do {
	expression: Expression,
}

impl Do {
	pub fn build(node: &SyntaxNode) -> Do {
		return Do {
			expression: Expression::build(&node.children()[node.children().len() - 1]),
		};
	}
}

impl Node for Do {
	fn execute(&self) {
		self.expression.execute();
	}
}
