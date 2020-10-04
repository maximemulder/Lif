use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Break<'a> {
	expression: Option<Node<'a>>,
}

impl<'a> Break<'a> {
	pub fn new(expression: Option<Node<'a>>) -> Self {
		return Self {
			expression,
		};
	}
}

impl<'a> Executable<'a> for Break<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return engine.control_new(Control::Break, &self.expression);
	}
}
