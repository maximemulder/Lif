use crate::runtime::{ Engine, Reference };
use super::expression::Expression;
use super::token::Token;
use super::{ Node, SyntaxNode };

pub struct Operation {
	left:     Expression,
	right:    Expression,
	operator: Box<str>,
}

impl Operation {
	pub fn build(node: &SyntaxNode) -> Operation {
		return Operation {
			left:     Expression::build(&node.children()[0]),
			operator: Token::build(&node.children()[1]),
			right:    Expression::build(&node.children()[2]),
		};
	}
}

impl Node for Operation {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		let left = self.left.execute(engine);
		let right = self.right.execute(engine);
		if self.operator.to_string() == "=" {
			engine.write(left, engine.read(right));
			return engine.new_undefined();
		} else {
			return engine.call(engine.read(engine.get_object(engine.read(left)).get_method(engine, &self.operator).unwrap()), vec![left, right]);
		}
	}
}
