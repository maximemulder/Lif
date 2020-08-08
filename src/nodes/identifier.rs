use super::SyntaxNode;

pub struct Identifier {
	pub text: Box<str>,
}

impl Identifier {
	pub fn build(node: &SyntaxNode) -> Identifier {
		return Identifier {
			text: Box::from(node.text()),
		};
	}
}
