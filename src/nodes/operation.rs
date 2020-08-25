use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		let mut left  = value!(self.left.execute(engine));
		let right = value!(self.right.execute(engine));
		if self.operator.to_string() == "=" {
			*left.value_mut() = *right.value_ref();
			return Product::new(Reference::new_undefined());
		}

		return Product::new(engine.call(*(left.object_ref().get_method(engine, &self.operator).unwrap()).value_ref(), vec![left, right]));
	}
}
