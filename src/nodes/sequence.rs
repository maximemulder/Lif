use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, Product };

pub struct Sequence {
	expression:  Expression,
	open:        Box<str>,
	expressions: Vec<Expression>,
	close:       Box<str>,
}

impl Sequence {
	pub fn new(expression:  Expression, open: Box<str>, expressions: Vec<Expression>, close: Box<str>) -> Self {
		return Self {
			expression,
			open,
			expressions,
			close,
		};
	}
}

impl Node for Sequence {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		let reference = value!(self.expression.execute(engine));
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(value!(argument.execute(engine)));
		}

		return Product::new(engine.call(*reference.value_ref(), arguments));
	}
}
