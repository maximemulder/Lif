use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;

pub struct Method<'a> {
	expression: Node<'a>,
	member:     &'a str,
	expressions: Box<[Node<'a>]>,
}

impl<'a> Method<'a> {
	pub fn new(expression: Node<'a>, member: &'a str, expressions: Box<[Node<'a>]>) -> Self {
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
		if let Some(method) = this.get_method(self.member) {
			let mut arguments = Vec::new();
			arguments.push(engine.new_constant(this));
			for argument in self.expressions.iter() {
				arguments.push(execute!(engine, argument));
			}

			return method.call(engine, arguments);
		}

		return Err(Error::new_undefined_method(&self.member, this.class));
	}
}
