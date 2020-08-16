use crate::runtime::Engine;
use super::expression::Expression;
use super::{ Node, SyntaxNode, Product };

pub struct Statement {
	expression: Expression,
}

impl Statement {
	pub fn build(node: &SyntaxNode) -> Statement {
		return Statement {
			expression: Expression::build(&node.children()[0]),
		};
	}
}

impl Node for Statement {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		value!(self.expression.execute(engine));
		return Product::new(engine.new_undefined());
	}
}
