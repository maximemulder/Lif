use crate::nodes::{ Node, Product };
use crate::runtime::Engine;

pub struct Identifier {
	identifier: Box<str>,
}

impl Identifier {
	pub fn new(identifier: Box<str>) -> Self {
		return Self {
			identifier,
		};
	}
}

impl Node for Identifier {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return Product::new(engine.get_variable(&self.identifier));
	}
}
