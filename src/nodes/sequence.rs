use super::expression::Expression;
use super::expressions::Expressions;
use super::token::Token;
use super::{ Engine, Node, SyntaxNode };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Option<usize> {
		let expression = self.expression.execute(engine);
		for argument in self.expressions.iter() {
			argument.execute(engine);
		}

		return None;
	}
}
