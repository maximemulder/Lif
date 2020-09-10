use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct DoWhile<'a> {
	node: &'a SyntaxNode<'a>,
	body:      Block<'a>,
	condition: Expression<'a>,
}

impl<'a> DoWhile<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, body: Block<'a>, condition: Expression<'a>) -> Self {
		return Self {
			node,
			body,
			condition,
		};
	}
}

impl Node for DoWhile<'_> {
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
