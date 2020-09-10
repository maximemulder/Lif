use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct If<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	condition: Expression<'a, 'b>,
	then:      Block<'a, 'b>,
	r#else:    Option<Block<'a, 'b>>,
}

impl<'a, 'b> If<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, condition: Expression<'a, 'b>, then: Block<'a, 'b>, r#else: Option<Block<'a, 'b>>) -> Self {
		return Self {
			node,
			condition,
			then,
			r#else,
		};
	}
}

impl Node for If<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return if {
			let reference = execute!(engine, &self.condition);
			*reference.read()?.get_cast_boolean(engine)?
		} {
			engine.execute(&self.then)
		} else if let Some(r#else) = self.r#else.as_ref() {
			engine.execute(r#else)
		} else {
			Ok(engine.new_undefined())
		}
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
