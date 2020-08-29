use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;

pub struct Declaration {
	identifier: Box<str>,
}

impl Declaration {
	pub fn new(identifier: Box<str>) -> Self {
		return Self {
			identifier,
		};
	}
}

impl Node for Declaration {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference<'a> {
		let reference = engine.new_undefined();
		engine.new_variable(&self.identifier, reference);
		return reference;
	}
}
