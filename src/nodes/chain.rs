use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, Product };

pub struct Chain {
	expression: Expression,
	member:     Box<str>,
}

impl Chain {
	pub fn new(expression: Expression, member: Box<str>) -> Self {
		return Self {
			expression,
			member,
		};
	}
}

impl Node for Chain {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let reference = value!(self.expression.execute(engine));
		let string = engine.new_string(self.member.to_string());
		return Product::new(engine.call_method(reference, ".", vec![string]));
	}
}
