use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::reference::GcReference;

pub struct Loop {
	body: Expression,
}

impl Loop {
	pub fn new(body: Expression) -> Self {
		return Self {
			body,
		};
	}
}

impl Node for Loop {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		let mut array = Vec::new();
		loop {
			let reference = self.body.execute(engine);
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
		}

		return engine.new_array(array);
	}
}
