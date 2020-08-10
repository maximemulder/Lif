use crate::runtime::{ Engine, Reference };
use super::expression::Expression;
use super::then::Then;
use super::r#else::Else;
use super::{ Node, SyntaxNode };

pub struct If {
	condition: Expression,
	then:      Expression,
	r#else:    Option<Expression>,
}

impl If {
	pub fn build(node: &SyntaxNode) -> If {
		return If {
			condition: Expression::build(&node.children()[0]),
			then:      Then::build(&node.children()[1]),
			r#else: if let Some(r#else) = node.children().get(2) {
				Some(Else::build(&r#else))
			} else {
				None
			},
		};
	}
}

impl Node for If {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		let reference = self.condition.execute(engine);
		let condition = engine.read(reference);
		return if engine.get_cast_boolean(condition) {
			self.then.execute(engine)
		} else if let Some(r#else) = &self.r#else {
			r#else.execute(engine)
		} else {
			return engine.new_undefined();
		}
	}
}
