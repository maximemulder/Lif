use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct While<'a> {
	node: &'a SyntaxNode<'a>,
	condition: Expression<'a>,
	body:      Block<'a>,
}

impl<'a> While<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, condition: Expression<'a>, body: Block<'a>) -> Self {
		return Self {
			node,
			condition,
			body,
		};
	}
}

impl Node for While<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let mut array = Vec::new();
		while {
			let reference = execute!(engine, &self.condition);
			*reference.read()?.get_cast_boolean(engine)?
		} {
			let reference = engine.execute(&self.body)?;
			if engine.control_is(Control::Return) {
				return Ok(reference);
			}

			array.push(reference);
			if engine.control_consume(Control::Break) {
				break;
			}

			if engine.control_consume(Control::Continue) {
				continue;
			}
		}

		return Ok(engine.new_array(array));
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
