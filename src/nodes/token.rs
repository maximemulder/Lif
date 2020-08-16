use super::SyntaxNode;

pub fn token(node: &SyntaxNode) -> Box<str> {
	return Box::from(node.text());
}
