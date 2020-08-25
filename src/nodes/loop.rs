use crate::runtime::engine::Engine;
use super::expression::Expression;
use super::{ Node, Product, Control };

pub struct Loop {
	body: Expression,
}

impl Loop {
	pub fn new(body: Expression) -> Self {
		return Self {
			body,
		};
	}
}

impl Node for Loop {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		let mut array = Vec::new();
		loop {
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
