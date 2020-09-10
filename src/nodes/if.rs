use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct If {
	condition: Expression,
	then:      Block,
	r#else:    Option<Block>,
}

impl If {
	pub fn new(condition: Expression, then: Block, r#else: Option<Block>) -> Self {
		return Self {
			condition,
			then,
			r#else,
		};
	}
}

impl Node for If {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return if {
			let reference = execute!(engine, &self.condition);
			*reference.read()?.get_cast_boolean(engine)?
		} {
			engine.execute(&self.then)
		} else if let Some(r#else) = self.r#else.as_ref() {
			engine.execute(r#else)
		} else {
			Ok(engine.new_undefined())
		}
	}
}
