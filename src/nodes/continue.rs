use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Continue {
	expression: Option<Expression>
}

impl Continue {
	pub fn new(expression: Option<Expression>) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Continue {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return engine.new_control(Control::Continue, &self.expression);
	}
}
