use super::expression::Expression;
use super::SyntaxNode;

pub struct Do;

impl Do {
	pub fn build(node: &SyntaxNode) -> Expression {
		return Expression::build(&node.children()[node.children().len() - 1]);
	}
}
