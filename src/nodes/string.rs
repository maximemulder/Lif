use crate::nodes::{ Node, SyntaxNode, Product };
use crate::runtime::Engine;

pub struct String {
	string: Box<str>,
}

impl String {
	pub fn build(node: &SyntaxNode) -> Self {
		let text = node.text();
		return Self {
			string: Box::from(&text[1 .. text.len() - 1]),
		};
	}
}

impl Node for String {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return Product::new(engine.new_string(self.string.to_string()));
	}
}
