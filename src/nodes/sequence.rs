use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Sequence<'a> {
	expression:  Node<'a>,
	open:        &'a str,
	expressions: Vec<Node<'a>>,
	close:       &'a str,
}

impl<'a> Sequence<'a> {
	pub fn new(expression: Node<'a>, open: &'a str, expressions: Vec<Node<'a>>, close: &'a str) -> Self {
		return Self {
			expression,
			open,
			expressions,
			close,
		};
	}
}

impl<'a> Executable<'a> for Sequence<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let value = execute!(engine, &self.expression).read()?;
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(execute!(engine, argument).read()?);
		}

		let mut name = String::new();
		name.push_str(&self.open);
		name.push_str(&self.close);
		return value.call_method(engine, &name, arguments);
	}
}
