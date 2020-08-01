use super::{ Node, SyntaxNode };

pub struct String {
	text: Box<str>,
}

impl String {
	pub fn build(node: &SyntaxNode) -> String {
		return String {
			text: Box::from(node.text()),
		};
	}
}

impl Node for String {
	fn execute(&self) {

	}
}
