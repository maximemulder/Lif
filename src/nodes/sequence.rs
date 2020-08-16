use crate::runtime::Engine;
use super::expression::Expression;
use super::expressions::expressions;
use super::token::token;
use super::{ Node, SyntaxNode, Product };

pub struct Sequence {
	expression:  Expression,
	open:        Box<str>,
	expressions: Vec<Expression>,
	close:       Box<str>,
}

impl Sequence {
	pub fn build(node: &SyntaxNode) -> Sequence {
		return Sequence {
			expression:  Expression::build(&node.children()[0]),
			open:        token(&node.children()[1]),
			expressions: expressions(&node.children()[2]),
			close:       token(&node.children()[3]),
		};
	}
}

impl Node for Sequence {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		let reference = value!(self.expression.execute(engine));
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(value!(argument.execute(engine)));
		}

		return Product::new(engine.call(engine.read(reference), arguments));
	}
}
