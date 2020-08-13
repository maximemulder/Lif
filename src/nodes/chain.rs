use crate::runtime::{ Engine, Reference };
use super::expression::Expression;
use super::token::Token;
use super::{ Node, SyntaxNode };

pub struct Chain {
	expression: Expression,
	member:     Box<str>,
}

impl Chain {
	pub fn build(node: &SyntaxNode) -> Chain {
		return Chain {
			expression: Expression::build(&node.children()[0]),
			member:     Token::build(&node.children()[2]),
		};
	}
}

impl Node for Chain {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		return engine.call_method(self.expression.execute(engine), ".", vec![engine.new_string(self.member.to_string())]);
	}
}
