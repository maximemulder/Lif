use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

pub struct Statement {
	node: Box<dyn Node>,
}

impl Statement {
	pub fn new(node: Box<dyn Node>) -> Self {
		return Self {
			node,
		};
	}
}

impl Node for Statement {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		execute!(engine, self.node.as_ref());
		return Ok(engine.new_undefined());
	}
}
