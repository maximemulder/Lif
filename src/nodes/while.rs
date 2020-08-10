use super::expression::Expression;
use super::r#do::Do;
use super::{ Engine, Node, SyntaxNode };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Option<usize> {
		let condition = self.condition.execute(engine).unwrap();
		while engine.get_cast_boolean(condition) {
			self.body.execute(engine);
		}

		return None;
	}
}
