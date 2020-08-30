use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;

pub struct Statement {
	expression: Expression,
}

impl Statement {
	pub fn new(expression: Expression) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Statement {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		execute!(engine, &self.expression);
		return engine.new_undefined();
	}
}
