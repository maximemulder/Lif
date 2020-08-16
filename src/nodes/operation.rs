use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, Product };

pub struct Operation {
	left:     Expression,
	right:    Expression,
	operator: Box<str>,
}

impl Operation {
	pub fn new(left: Expression, right: Expression, operator: Box<str>) -> Self {
		return Self {
			left,
			right,
			operator,
		};
	}
}

impl Node for Operation {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let left  = value!(self.left.execute(engine));
		let right = value!(self.right.execute(engine));
		if self.operator.to_string() == "=" {
			engine.write(left, engine.read(right));
			return Product::new(engine.new_undefined());
		}

		return Product::new(engine.call(engine.read(engine.get_object(engine.read(left)).get_method(engine, &self.operator).unwrap()), vec![left, right]));
	}
}
