use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		value!(self.expression.execute(engine));
		return Product::new(Reference::new_undefined());
	}
}
