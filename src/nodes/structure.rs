use crate::runtime::Engine;
use super::{ Node, Product };

pub struct Structure {
	node: Box<dyn Node>,
}

impl Structure {
	pub fn new(node: Box<dyn Node>) -> Self {
		return Self {
			node,
		};
	}
}

impl Node for Structure {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return self.node.execute(engine);
	}
}
