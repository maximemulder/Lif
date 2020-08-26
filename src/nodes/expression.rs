use crate::runtime::engine::Engine;
use super::{ Node, Product };

pub struct Expression {
	node: Box<dyn Node>,
}

impl Expression {
	pub fn new(node: Box<dyn Node>) -> Self {
		return Self {
			node,
		};
	}
}

impl Node for Expression {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		return self.node.execute(engine);
	}
}
