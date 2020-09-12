use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Loop<'a> {
	node: &'a SyntaxNode<'a>,
	body: Block<'a>,
}

impl<'a> Loop<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, body: Block<'a>) -> Self {
		return Self {
			node,
			body,
		};
	}
}

impl Node for Loop<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let mut array = Vec::new();
		loop {
			let reference = engine.execute(&self.body)?;
			if engine.control_is(Control::Return) {
				return Ok(reference);
			}

			if reference.is_defined() {
				array.push(engine.new_reference(reference.get_value()));
			}

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
