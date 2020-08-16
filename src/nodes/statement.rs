use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, Product };

pub struct Statement {
	expression: Expression,
}

impl Statement {
	pub fn new(expression: Expression) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Statement {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		value!(self.expression.execute(engine));
		return Product::new(engine.new_undefined());
	}
}
