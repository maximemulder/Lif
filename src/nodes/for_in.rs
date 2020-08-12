use crate::runtime::{ Engine, Reference };
use super::{ Node, SyntaxNode };
use super::expression::Expression;
use super::r#do::Do;
use super::token::Token;

pub struct ForIn {
	identifier: Box<str>,
	expression: Expression,
	body:       Expression,
}

impl ForIn {
	pub fn build(node: &SyntaxNode) -> ForIn {
		return ForIn {
			identifier: Token::build(&node.children()[1]),
			expression: Expression::build(&node.children()[3]),
			body:       Do::build(&node.children()[4]),
		};
	}
}

impl Node for ForIn {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		for element in engine.get_cast_array(engine.read(self.expression.execute(engine))) {
			engine.new_variable(&self.identifier, *element);
			self.body.execute(engine);
		}

		return engine.new_undefined();
	}
}
