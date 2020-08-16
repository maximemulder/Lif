use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, Product };

pub struct If {
	condition: Expression,
	then:      Expression,
	r#else:    Option<Expression>,
}

impl If {
	pub fn new(condition: Expression, then: Expression, r#else: Option<Expression>) -> Self {
		return Self {
			condition,
			then,
			r#else,
		};
	}
}

impl Node for If {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return if {
			let reference = value!(self.condition.execute(engine));
			*engine.get_cast_boolean(engine.read(reference))
		} {
			self.then.execute(engine)
		} else if let Some(r#else) = &self.r#else {
			r#else.execute(engine)
		} else {
			return Product::new(engine.new_undefined());
		}
	}
}
