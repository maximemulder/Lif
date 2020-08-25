use crate::nodes::{ Node, Product };
use crate::runtime::engine::Engine;

pub struct Integer {
	integer: usize,
}

impl Integer {
	pub fn new(integer: usize) -> Self {
		return Self {
			integer,
		};
	}
}

impl Node for Integer {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		return Product::new(engine.new_integer(self.integer));
	}
}
