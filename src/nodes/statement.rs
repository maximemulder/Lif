use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		execute!(engine, self.node.as_ref());
		return Ok(engine.new_undefined());
	}
}
