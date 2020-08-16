use crate::runtime::Engine;
use super::expression::Expression;
use super::token::token;
use super::{ Node, SyntaxNode, Product };

pub struct Operation {
	left:     Expression,
	right:    Expression,
	operator: Box<str>,
}

impl Operation {
	pub fn build(node: &SyntaxNode) -> Operation {
		return Operation {
			left:     Expression::build(&node.children()[0]),
			operator: token(&node.children()[1]),
			right:    Expression::build(&node.children()[2]),
		};
	}
}

impl Node for Operation {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let left  = value!(self.left.execute(engine));
		let right = value!(self.right.execute(engine));
		if self.operator.to_string() == "=" {
			engine.write(left, engine.read(right));
			return Product::new(engine.new_undefined());
		}

		return Product::new(engine.call(engine.read(engine.get_object(engine.read(left)).get_method(engine, &self.operator).unwrap()), vec![left, right]));
	}
}
