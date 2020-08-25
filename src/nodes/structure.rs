use crate::runtime::engine::Engine;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		return self.node.execute(engine);
	}
}
