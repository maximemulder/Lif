use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		let r#type = if let Some(r#type) = &self.r#type {
			let value = execute!(engine, r#type).read()?;
			value.cast(engine.environment.class)?;
			value
		} else {
			engine.environment.object
		};

		let reference = engine.new_variable(None, r#type);
		engine.add_variable(&self.identifier, reference);
		return Ok(reference);
	}
}
