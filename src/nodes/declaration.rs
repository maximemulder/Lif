use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Declaration<'a> {
	node: &'a SyntaxNode<'a>,
	identifier: &'a str,
	r#type: Option<Expression<'a>>,
}

impl<'a> Declaration<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, identifier: &'a str, r#type: Option<Expression<'a>>) -> Self {
		return Self {
			node,
			identifier,
			r#type,
		};
	}
}

impl<'a> Node<'a> for Declaration<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let r#type = if let Some(r#type) = &self.r#type {
			let value = execute!(engine, r#type).read()?;
			value.cast(engine.environment.class)?;
			value
		} else {
			engine.environment.object
		};

		let reference = engine.new_variable(None, r#type);
		engine.add_variable(&self.identifier, reference);
		return Ok(reference);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
