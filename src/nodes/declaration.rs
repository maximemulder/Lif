use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Declaration<'a> {
	node: &'a SyntaxNode<'a>,
	identifier: Box<str>,
	r#type: Option<Expression<'a>>,
}

impl<'a> Declaration<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, identifier: Box<str>, r#type: Option<Expression<'a>>) -> Self {
		return Self {
			node,
			identifier,
			r#type,
		};
	}
}

impl Node for Declaration<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
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

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
