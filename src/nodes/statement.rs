use crate::nodes::Node;
use crate::runtime::engine::Engine;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		execute!(engine, self.node.as_ref());
		return engine.new_undefined();
	}
}
