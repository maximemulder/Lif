use crate::runtime::{ Engine, Reference };
use super::expression::Expression;
use super::expressions::Expressions;
use super::token::Token;
use super::{ Node, SyntaxNode };

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
			open:        Token::build(&node.children()[1]),
			expressions: Expressions::build(&node.children()[2]),
			close:       Token::build(&node.children()[3]),
		};
	}
}

impl Node for Sequence {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		let mut arguments = Vec::new();
		let reference = self.expression.execute(engine);
		for argument in self.expressions.iter() {
			arguments.push(argument.execute(engine));
		}

		return engine.call(engine.read(reference), arguments);
	}
}
