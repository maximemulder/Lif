use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

pub struct Break {
	expression: Option<Expression>
}

impl Break {
	pub fn new(expression: Option<Expression>) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Break {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		return engine.new_control(Control::Break, &self.expression);
	}
}
