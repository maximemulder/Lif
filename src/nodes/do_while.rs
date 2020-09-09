use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

pub struct DoWhile {
	body:      Block,
	condition: Expression,
}

impl DoWhile {
	pub fn new(body: Block, condition: Expression) -> Self {
		return Self {
			body,
			condition,
		};
	}
}

impl Node for DoWhile {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		let mut array = Vec::new();
		loop {
			let reference = engine.execute(&self.body)?;
			match &engine.control {
				Some(control) => match control {
					Control::Return => return Ok(reference),
					Control::Continue => {
						engine.control = None;
						array.push(reference);
						continue;
					},
					Control::Break => {
						engine.control = None;
						array.push(reference);
						break
					},
				},
				None => array.push(reference),
			}

			if {
				let reference = execute!(engine, &self.condition);
				!*reference.read()?.get_cast_boolean(engine)?
			} {
				break;
			}
		}

		return Ok(engine.new_array(array));
	}
}
