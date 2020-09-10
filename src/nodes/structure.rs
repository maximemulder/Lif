use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.execute(self.node.as_ref());
	}
}
