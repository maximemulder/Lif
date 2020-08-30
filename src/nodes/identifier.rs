use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		return engine.get_variable(&self.identifier);
	}
}
