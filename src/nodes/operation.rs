use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Operation<'a, 'b> {
	node: &'b SyntaxNode<'a>,
	left:     Expression<'a, 'b>,
	right:    Expression<'a, 'b>,
	operator: Box<str>,
}

impl<'a, 'b> Operation<'a, 'b> {
	pub fn new(node: &'b SyntaxNode<'a>, left: Expression<'a, 'b>, right: Expression<'a, 'b>, operator: Box<str>) -> Self {
		return Self {
			node,
			left,
			right,
			operator,
		};
	}
}

impl Node for Operation<'_, '_> {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		if self.operator.to_string() == "=" {
			let mut left  = execute!(engine, &self.left);
			let right = execute!(engine, &self.right).read()?;
			left.write(right)?;
			return Ok(engine.new_undefined());
		}

		let left  = execute!(engine, &self.left).read()?;
		let right = execute!(engine, &self.right).read()?;

		return engine.call((left.get_method(engine, &self.operator).unwrap()).read()?, vec![left, right]);
	}

	fn get_syntax_node(&self) -> &SyntaxNode {
		return self.node;
	}
}
