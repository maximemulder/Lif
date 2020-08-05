use super::expression::Expression;
use super::{ Node, SyntaxNode };

pub struct Else {
	expression: Expression,
}

impl Else {
	pub fn build(node: &SyntaxNode) -> Else {
		return Else {
			expression: Expression::build(&node.children()[1]),
		};
	}
}

impl Node for Else {
	fn execute(&self) {
		self.expression.execute();
	}
}
