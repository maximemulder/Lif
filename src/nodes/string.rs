use super::SyntaxNode;

pub struct String {
	pub text: Box<str>,
}

impl String {
	pub fn build(node: &SyntaxNode) -> String {
		return String {
			text: Box::from(node.text()),
		};
	}
}
