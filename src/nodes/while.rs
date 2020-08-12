use super::expression::Expression;
use super::r#do::Do;
use crate::runtime::{ Engine, Reference };
use super::{ Node, SyntaxNode };

pub struct While {
	condition: Expression,
	body:      Expression,
}

impl While {
	pub fn build(node: &SyntaxNode) -> While {
		return While {
			condition: Expression::build(&node.children()[1]),
			body:      Do::build(&node.children()[2]),
		};
	}
}

impl Node for While {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		while *engine.get_cast_boolean(engine.read(self.condition.execute(engine))) {
			self.body.execute(engine);
		}

		return engine.new_undefined();
	}
}
