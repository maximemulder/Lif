use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Operation<'a> {
	node: &'a SyntaxNode<'a>,
	left:     Expression<'a>,
	right:    Expression<'a>,
	operator: &'a str,
}

impl<'a> Operation<'a> {
	pub fn new(node: &'a SyntaxNode<'a>, left: Expression<'a>, right: Expression<'a>, operator: &'a str) -> Self {
		return Self {
			node,
			left,
			right,
			operator,
		};
	}
}

impl<'a> Node<'a> for Operation<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		if self.operator.to_string() == "=" {
			let mut left  = execute!(engine, &self.left);
			let right = execute!(engine, &self.right).read()?;
			left.write(right)?;
			return Ok(engine.undefined());
		}

		let left  = execute!(engine, &self.left).read()?;
		let right = execute!(engine, &self.right).read()?;

		return engine.call((left.get_method(engine, &self.operator).unwrap()).read()?, vec![left, right]);
	}

	fn get_syntax_node(&self) -> &'a SyntaxNode<'a> {
		return self.node;
	}
}
