use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Group {
	expression: Expression,
}

impl Group {
	pub fn new(expression: Expression) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Group {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.execute(&self.expression);
	}
}
