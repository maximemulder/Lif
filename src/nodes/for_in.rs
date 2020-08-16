use crate::runtime::{ Engine, Reference };
use super::{ Node, SyntaxNode };
use super::expression::Expression;
use super::r#do::r#do;
use super::token::token;

pub struct ForIn {
	identifier: Box<str>,
	expression: Expression,
	body:       Expression,
}

impl ForIn {
	pub fn build(node: &SyntaxNode) -> ForIn {
		return ForIn {
			identifier: token(&node.children()[1]),
			expression: Expression::build(&node.children()[3]),
			body:       r#do(&node.children()[4]),
		};
	}
}

impl Node for ForIn {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		for element in {
			let reference = self.expression.execute(engine);
			engine.get_cast_array(engine.read(reference)).clone()
		} {
			engine.new_variable(&self.identifier, element);
			self.body.execute(engine);
		}

		return engine.new_undefined();
	}
}
