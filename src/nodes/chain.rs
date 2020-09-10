use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Chain<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	expression: Expression<'a, 'b>,
	member:     Box<str>,
}

impl<'a, 'b> Chain<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, expression: Expression<'a, 'b>, member: Box<str>) -> Self {
		return Self {
			node,
			expression,
			member,
		};
	}
}

impl Node for Chain<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let value = execute!(engine, &self.expression).read()?;
		let name = engine.new_string(self.member.to_string()).read()?;
		return engine.call_method(value, ".", vec![name]);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
