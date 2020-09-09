use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		return engine.execute(&self.expression);
	}
}
