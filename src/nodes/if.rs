use crate::runtime::Engine;
use super::expression::Expression;
use super::then::then;
use super::r#else::r#else;
use super::{ Node, SyntaxNode, Product };

pub struct If {
	condition: Expression,
	then:      Expression,
	r#else:    Option<Expression>,
}

impl If {
	pub fn build(node: &SyntaxNode) -> If {
		return If {
			condition: Expression::build(&node.children()[0]),
			then:      then(&node.children()[1]),
			r#else: if let Some(child) = node.children().get(2) {
				Some(r#else(child))
			} else {
				None
			},
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
