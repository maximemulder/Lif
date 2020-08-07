use super::expression::Expression;
use super::{ Engine, Node, SyntaxNode };

pub struct Expressions {
	expressions: Vec<Expression>,
}

impl Expressions {
	pub fn build(node: &SyntaxNode) -> Expressions {
		let mut expressions = Vec::new();
		for (i, child) in node.children().iter().enumerate()  {
			if i % 2 == 1 {
				continue;
			}

			expressions.push(Expression::build(child));
		}

		return Expressions {
			expressions,
		};
	}
}

impl Node for Expressions {
	fn execute(&self, engine: &mut Engine) {

	}
}
