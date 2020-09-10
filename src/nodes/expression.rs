use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		engine.collect();
		return engine.execute(self.node.as_ref());
	}
}
