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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		let reference = self.condition.execute(engine);
		let condition = engine.read(reference);
		while engine.get_cast_boolean(condition) {
			self.body.execute(engine);
		}

		return engine.new_undefined();
	}
}
