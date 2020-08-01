use super::{ Node, SyntaxNode };

pub struct Number {
	text: Box<str>,
}

impl Number {
	pub fn build(node: &SyntaxNode) -> Number {
		return Number {
			text: Box::from(node.text()),
		};
	}
}

impl Node for Number {
	fn execute(&self) {

	}
}
