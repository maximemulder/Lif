use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Chain<'a> {
	expression: Node<'a>,
	member:     &'a str,
}

impl<'a> Chain<'a> {
	pub fn new(expression: Node<'a>, member: &'a str) -> Self {
		return Self {
			expression,
			member,
		};
	}
}

impl<'a> Executable<'a> for Chain<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let value = execute!(engine, &self.expression).read()?;
		let name = engine.new_string(self.member.to_string());
		return value.call_method(engine, ".", vec![name]);
	}
}
