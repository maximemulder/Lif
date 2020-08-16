use crate::nodes::{ Node, SyntaxNode, Product };
use crate::runtime::Engine;

pub struct Integer {
	integer: usize,
}

impl Integer {
	pub fn build(node: &SyntaxNode) -> Self {
		return Self {
			integer: node.text().parse::<usize>().unwrap(),
		};
	}
}

impl Node for Integer {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return Product::new(engine.new_integer(self.integer));
	}
}
