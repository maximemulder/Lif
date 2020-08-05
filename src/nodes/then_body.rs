use super::expression::Expression;
use super::{ Node, SyntaxNode };

pub struct ThenBody {
	expression: Expression,
}

impl ThenBody {
	pub fn build(node: &SyntaxNode) -> ThenBody {
		return ThenBody {
			expression: Expression::build(&node.children()[node.children().len() - 1]),
		};
	}
}

impl Node for ThenBody {
	fn execute(&self) {
		self.expression.execute();
	}
}
