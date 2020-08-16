use super::expression::Expression;
use super::r#do::r#do;
use crate::runtime::Engine;
use super::{ Node, SyntaxNode, Product, Control };

pub struct While {
	condition: Expression,
	body:      Expression,
}

impl While {
	pub fn build(node: &SyntaxNode) -> While {
		return While {
			condition: Expression::build(&node.children()[1]),
			body:      r#do(&node.children()[2]),
		};
	}
}

impl Node for While {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let mut array = Vec::new();
		while {
			let reference = value!(self.condition.execute(engine));
			*engine.get_cast_boolean(engine.read(reference))
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
