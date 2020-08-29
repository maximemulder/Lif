use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::reference::Reference;

pub struct DoWhile {
	body:      Expression,
	condition: Expression,
}

impl DoWhile {
	pub fn new(body: Expression, condition: Expression) -> Self {
		return Self {
			body,
			condition,
		};
	}
}

impl Node for DoWhile {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference<'a> {
		let mut array = Vec::new();
		loop {
			let reference = engine.execute(&self.body);
			match &engine.control {
				Some(control) => match control {
					Control::Return => return reference,
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
				!reference.value_ref().get_cast_boolean(engine)
			} {
				break;
			}
		}

		return engine.new_array(array);
	}
}
