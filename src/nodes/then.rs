use super::expression::Expression;
use super::SyntaxNode;

pub struct Then;

impl Then {
	pub fn build(node: &SyntaxNode) -> Expression {
		return Expression::build(&node.children()[node.children().len() - 1]);
	}
}
