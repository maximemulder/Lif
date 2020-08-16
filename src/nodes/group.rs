use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, Product };

pub struct Group {
	expression: Expression,
}

impl Group {
	pub fn new(expression: Expression) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Group {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return self.expression.execute(engine);
	}
}
