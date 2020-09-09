use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		engine.collect();
		return engine.execute(self.node.as_ref());
	}
}
