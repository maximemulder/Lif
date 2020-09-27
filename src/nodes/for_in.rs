use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct ForIn<'a> {
	node: &'a SyntaxNode<'a>,
	identifier: Box<str>,
	expression: Expression<'a>,
	body:       Block<'a>,
}

impl<'a> ForIn<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, identifier: Box<str>, expression: Expression<'a>, body: Block<'a>) -> Self {
		return Self {
			node,
			identifier,
			expression,
			body,
		};
	}
}

impl<'a> Node<'a> for ForIn<'a> {
	fn execute(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let mut array = Vec::new();
		for element in {
			let reference = execute!(engine, &self.expression);
			reference.read()?.get_cast_array(engine)?.clone()
		} {
			engine.add_variable(&self.identifier, element);
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

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
