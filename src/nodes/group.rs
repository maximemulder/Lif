use super::expression::Expression;
use super::{ Engine, Node, SyntaxNode };

pub struct Group {
	expression: Expression,
}

impl Group {
	pub fn build(node: &SyntaxNode) -> Group {
		return Group {
			expression: Expression::build(&node.children()[node.children().len() - 1]),
		};
	}
}

impl Node for Group {
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		return self.expression.execute(engine);
	}
}
