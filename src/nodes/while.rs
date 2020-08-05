use super::expression::Expression;
use super::loop_body::LoopBody;
use super::{ Node, SyntaxNode };

pub struct While {
	condition: Expression,
	body:      LoopBody,
}

impl While {
	pub fn build(node: &SyntaxNode) -> While {
		return While {
			condition: Expression::build(&node.children()[1]),
			body:      LoopBody::build(&node.children()[2]),
		};
	}
}

impl Node for While {
	fn execute(&self) {
		self.condition.execute();
	}
}
