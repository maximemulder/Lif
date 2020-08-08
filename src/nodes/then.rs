use super::expression::Expression;
use super::{ Engine, Node, SyntaxNode };

pub struct Then {
	expression: Expression,
}

impl Then {
	pub fn build(node: &SyntaxNode) -> Then {
		return Then {
			expression: Expression::build(&node.children()[node.children().len() - 1]),
		};
	}
}

impl Node for Then {
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		return self.expression.execute(engine);
	}
}
