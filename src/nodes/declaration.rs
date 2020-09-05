use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct Declaration {
	identifier: Box<str>,
	r#type: Option<Expression>,
}

impl Declaration {
	pub fn new(identifier: Box<str>, r#type: Option<Expression>) -> Self {
		return Self {
			identifier,
			r#type,
		};
	}
}

impl Node for Declaration {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		let reference = engine.new_reference(None, true);
		engine.add_variable(&self.identifier, reference);
		return reference;
	}
}
