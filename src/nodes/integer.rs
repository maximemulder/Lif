use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		return engine.new_integer(self.integer);
	}
}
