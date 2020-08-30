use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;

pub struct If {
	condition: Expression,
	then:      Expression,
	r#else:    Option<Expression>,
}

impl If {
	pub fn new(condition: Expression, then: Expression, r#else: Option<Expression>) -> Self {
		return Self {
			condition,
			then,
			r#else,
		};
	}
}

impl Node for If {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		return if {
			let reference = execute!(engine, &self.condition);
			*reference.read().get_cast_boolean(engine)
		} {
			execute!(engine, &self.then)
		} else if let Some(r#else) = &self.r#else {
			execute!(engine, r#else)
		} else {
			return engine.new_undefined();
		}
	}
}
