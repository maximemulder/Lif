use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		let reference = engine.new_reference(None, true);
		engine.add_variable(&self.identifier, reference);
		return reference;
	}
}
