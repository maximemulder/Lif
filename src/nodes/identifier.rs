use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.get_variable(&self.identifier);
	}
}
