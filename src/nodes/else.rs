use super::expression::Expression;
use super::SyntaxNode;

pub struct Else;

impl Else {
	pub fn build(node: &SyntaxNode) -> Expression {
		return Expression::build(&node.children()[1]);
	}
}
