use super::expression::Expression;
use super::symbol::Symbol;
use super::{ Node, SyntaxNode };

pub struct Operation {
	left:     Expression,
	right:    Expression,
	operator: Symbol,
}

impl Operation {
	pub fn build(node: &SyntaxNode) -> Operation {
		return Operation {
			left:     Expression::build(&node.children()[1]),
			operator: Symbol::build(&node.children()[1]),
			right:    Expression::build(&node.children()[1]),
		};
	}
}

impl Node for Operation {
	fn execute(&self) {
		self.left.execute();
	}
}
