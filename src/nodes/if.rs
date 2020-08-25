use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		return if {
			let reference = value!(self.condition.execute(engine));
			*reference.value_ref().get_cast_boolean(engine)
		} {
			self.then.execute(engine)
		} else if let Some(r#else) = &self.r#else {
			r#else.execute(engine)
		} else {
			return Product::new(Reference::new_undefined());
		}
	}
}
