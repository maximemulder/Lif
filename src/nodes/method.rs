use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;

pub struct Method<'a> {
	expression: Node<'a>,
	member:     &'a str,
	expressions: Vec<Node<'a>>,
}

impl<'a> Method<'a> {
	pub fn new(expression: Node<'a>, member: &'a str, expressions: Vec<Node<'a>>) -> Self {
		return Self {
			expression,
			member,
			expressions,
		};
	}
}

impl<'a> Executable<'a> for Method<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let this = execute!(engine, &self.expression).read()?;
		if let Some(method) = this.get_method(engine, self.member) {
			let mut arguments = Vec::new();
			arguments.push(this);
			for argument in self.expressions.iter() {
				arguments.push(execute!(engine, argument).read()?);
			}

			return engine.call(method, arguments);
		}

		return Err(Error::new_runtime("Method does not exist."));
	}
}
