use crate::runtime::Engine;
use super::{ Node, Product };
use super::expression::Expression;

pub struct ForIn {
	identifier: Box<str>,
	expression: Expression,
	body:       Expression,
}

impl ForIn {
	pub fn new(	identifier: Box<str>, expression: Expression, body: Expression) -> Self {
		return Self {
			identifier,
			expression,
			body,
		};
	}
}

impl Node for ForIn {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		for element in {
			let reference = value!(self.expression.execute(engine));
			engine.get_cast_array(engine.read(reference)).clone()
		} {
			engine.new_variable(&self.identifier, element);
			self.body.execute(engine);
		}

		return Product::new(engine.new_undefined());
	}
}
