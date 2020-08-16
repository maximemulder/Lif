use crate::runtime::Engine;
use super::{ Node, Product, Control };
use super::expression::Expression;

pub struct Break {
	expression: Option<Expression>
}

impl Break {
	pub fn new(expression: Option<Expression>) -> Self {
		return Self {
			expression,
		};
	}
}

impl Node for Break {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return Product::new_control(if let Some(expression) = &self.expression {
			value!(expression.execute(engine))
		} else {
			engine.new_undefined()
		}, Control::Break);
	}
}
