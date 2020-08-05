use super::expression::Expression;
use super::{ Node, SyntaxNode };

pub struct LoopBody {
	expression: Expression,
}

impl LoopBody {
	pub fn build(node: &SyntaxNode) -> LoopBody {
		return LoopBody {
			expression: Expression::build(&node.children()[node.children().len() - 1]),
		};
	}
}

impl Node for LoopBody {
	fn execute(&self) {
		self.expression.execute();
	}
}
