use super::expression::Expression;
use super::symbol::Symbol;
use super::{ Node, SyntaxNode };

pub struct Operation {
	expression: Expression,
	open:       Symbol,
	close:      Symbol,
}

impl Operation {
	pub fn build(node: &SyntaxNode) -> Operation {
		return Operation {
			expression: Expression::build(&node.children()[0]),
			open:       Symbol::build(&node.children()[1]),
			close:      Symbol::build(&node.children()[3]),
		};
	}
}

impl Node for Operation {
	fn execute(&self) {
		loop {
			self.expression.execute();
		}
	}
}
