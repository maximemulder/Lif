use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct DoWhile<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	body:      Block<'a, 'b>,
	condition: Expression<'a, 'b>,
}

impl<'a, 'b> DoWhile<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, body: Block<'a, 'b>, condition: Expression<'a, 'b>) -> Self {
		return Self {
			node,
			body,
			condition,
		};
	}
}

impl Node for DoWhile<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
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

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
