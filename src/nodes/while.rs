use super::expression::Expression;
use crate::runtime::Engine;
use super::{ Node, Product, Control };

pub struct While {
	condition: Expression,
	body:      Expression,
}

impl While {
	pub fn new(condition: Expression, body: Expression) -> Self {
		return Self {
			condition,
			body,
		};
	}
}

impl Node for While {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		let mut array = Vec::new();
		while {
			let reference = value!(self.condition.execute(engine));
			*reference.object_ref().get_cast_boolean(engine)
		} {
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
