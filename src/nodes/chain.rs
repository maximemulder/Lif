use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct Chain {
	expression: Expression,
	member:     Box<str>,
}

impl Chain {
	pub fn new(expression: Expression, member: Box<str>) -> Self {
		return Self {
			expression,
			member,
		};
	}
}

impl Node for Chain {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		let value = execute!(engine, &self.expression).read();
		let name = engine.new_string(self.member.to_string()).read();
		return engine.call_method(value, ".", vec![name]);
	}
}
