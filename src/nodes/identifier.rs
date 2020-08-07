use super::{ Engine, Node, SyntaxNode };

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

impl Node for Identifier {
	fn execute(&self, engine: &mut Engine) {

	}
}
