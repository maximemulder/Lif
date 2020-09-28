use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Chain<'a> {
	node: &'a SyntaxNode<'a>,
	expression: Expression<'a>,
	member:     Box<str>,
}

impl<'a> Chain<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, expression: Expression<'a>, member: Box<str>) -> Self {
		return Self {
			node,
			expression,
			member,
		};
	}
}

impl<'a> Node<'a> for Chain<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		let value = execute!(engine, &self.expression).read()?;
		let name = engine.new_string(self.member.to_string()).read()?;
		return engine.call_method(value, ".", vec![name]);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
