use crate::runtime::Engine;
use super::{ Node, Product, Control };
use super::expression::Expression;

pub struct ForIn {
	identifier: Box<str>,
	expression: Expression,
	body:       Expression,
}

impl ForIn {
	pub fn new(	identifier: Box<str>, expression: Expression, body: Expression) -> Self {
		return Self {
			identifier,
			expression,
			body,
		};
	}
}

impl Node for ForIn {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		let mut array = Vec::new();
		for element in {
			let reference = value!(self.expression.execute(engine));
			reference.object_ref().get_cast_array(engine).clone()
		} {
			engine.new_variable(&self.identifier, element);
			let product = self.body.execute(engine);
			match &product.control {
				Some(control) => match control {
					Control::Return => return product,
					Control::Continue => {
						array.push(product.reference);
						continue;
					},
					Control::Break => {
						array.push(product.reference);
						break
					},
				},
				None => array.push(product.reference),
			}
		}

		return Product::new(engine.new_array(array));
	}
}
