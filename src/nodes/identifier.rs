use crate::nodes::{ Node, SyntaxNode, Product };
use crate::runtime::Engine;

pub struct Identifier {
	identifier: Box<str>,
}

impl Identifier {
	pub fn build(node: &SyntaxNode) -> Self {
		return Self {
			identifier: Box::from(node.text()),
		};
	}
}

impl Node for Identifier {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return Product::new(engine.get_variable(&self.identifier));
	}
}
