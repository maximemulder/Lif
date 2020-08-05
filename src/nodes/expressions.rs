use super::expression::Expression;
use super::{ Node, SyntaxNode };

pub struct Else {
	expressions: Vec<Expression>,
}

impl Else {
	pub fn build(node: &SyntaxNode) -> Else {
		let mut expressions = Vec::new();
		for (i, child) in node.children().iter().enumerate()  {
			if i % 2 == 1 {
				continue;
			}

			expressions.push(Expression::build(child));
		}

		return Else {
			expressions,
		};
	}
}

impl Node for Else {
	fn execute(&self) {

	}
}
