use super::SyntaxNode;

pub struct Token;

impl Token {
	pub fn build(node: &SyntaxNode) -> Box<str> {
		return Box::from(node.text());
	}
}
