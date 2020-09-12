use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct If<'a> {
	node: &'a SyntaxNode<'a>,
	condition: Expression<'a>,
	then:      Block<'a>,
	r#else:    Option<Block<'a>>,
}

impl<'a> If<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, condition: Expression<'a>, then: Block<'a>, r#else: Option<Block<'a>>) -> Self {
		return Self {
			node,
			condition,
			then,
			r#else,
		};
	}
}

impl Node for If<'_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return if {
			let reference = execute!(engine, &self.condition);
			*reference.read()?.get_cast_boolean(engine)?
		} {
			engine.execute(&self.then)
		} else if let Some(r#else) = self.r#else.as_ref() {
			engine.execute(r#else)
		} else {
			Ok(engine.undefined())
		}
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
