use crate::runtime::Engine;
use super::expression::Expression;
use super::token::token;
use super::{ Node, SyntaxNode, Product };

pub struct Chain {
	expression: Expression,
	member:     Box<str>,
}

impl Chain {
	pub fn build(node: &SyntaxNode) -> Chain {
		return Chain {
			expression: Expression::build(&node.children()[0]),
			member:     token(&node.children()[2]),
		};
	}
}

impl Node for Chain {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let reference = value!(self.expression.execute(engine));
		let string = engine.new_string(self.member.to_string());
		return Product::new(engine.call_method(reference, ".", vec![string]));
	}
}
