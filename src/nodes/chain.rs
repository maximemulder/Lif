use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference<'a> {
		let reference = execute!(engine, &self.expression);
		let string = engine.new_string(self.member.to_string());
		return engine.call_method(reference, ".", vec![string]);
	}
}
