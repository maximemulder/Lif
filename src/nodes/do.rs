use super::expression::Expression;
use super::SyntaxNode;

pub fn r#do(node: &SyntaxNode) -> Expression {
	return Expression::build(&node.children()[node.children().len() - 1]);
}
