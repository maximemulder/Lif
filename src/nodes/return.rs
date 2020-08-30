use crate::runtime::engine::{ Control, Engine };
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		return engine.new_control(Control::Return, &self.expression);
	}
}
