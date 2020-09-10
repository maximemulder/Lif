use crate::nodes::Node;
use crate::runtime::ReturnReference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return Ok(engine.new_integer(self.integer));
	}
}
