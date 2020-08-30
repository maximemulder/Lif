use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		engine.collect();
		return execute!(engine, self.node.as_ref());
	}
}
