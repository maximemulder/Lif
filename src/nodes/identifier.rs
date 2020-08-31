use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		return engine.get_variable(&self.identifier);
	}
}
