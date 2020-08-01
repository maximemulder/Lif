use super::{ Node, SyntaxNode };

pub struct Identifier {
	text: Box<str>,
}

impl Identifier {
	pub fn build(node: &SyntaxNode) -> Identifier {
		return Identifier {
			text: Box::from(node.text()),
		};
	}
}

impl Node for Identifier {
	fn execute(&self) {

	}
}
