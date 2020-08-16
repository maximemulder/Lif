use super::expression::Expression;
use super::SyntaxNode;

pub fn r#else(node: &SyntaxNode) -> Expression {
	return Expression::build(&node.children()[1]);
}
