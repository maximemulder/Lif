use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::reference::GcReference;

pub struct ForIn {
	identifier: Box<str>,
	expression: Expression,
	body:       Block,
}

impl ForIn {
	pub fn new(	identifier: Box<str>, expression: Expression, body: Block) -> Self {
		return Self {
			identifier,
			expression,
			body,
		};
	}
}

impl Node for ForIn {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		let mut array = Vec::new();
		for element in {
			let reference = execute!(engine, &self.expression);
			reference.read().get_cast_array(engine).clone()
		} {
			engine.add_variable(&self.identifier, element);
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
		}

		return engine.new_array(array);
	}
}
