use super::SyntaxNode;

pub struct Number {
	pub text: Box<str>,
}

impl Number {
	pub fn build(node: &SyntaxNode) -> Number {
		return Number {
			text: Box::from(node.text()),
		};
	}
}

