use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::nodes::Node;
use crate::nodes::expression::Expression;

pub struct Return {
	expression: Option<Expression>
}

impl Return {
	pub fn new(expression: Option<Expression>) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Return {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		return engine.new_control(Control::Return, &self.expression);
	}
}
