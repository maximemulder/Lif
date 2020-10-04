use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Operation<'a> {
	left:     Node<'a>,
	right:    Node<'a>,
	operator: &'a str,
}

impl<'a> Operation<'a> {
	pub fn new(left: Node<'a>, right: Node<'a>, operator: &'a str) -> Self {
		return Self {
			left,
			right,
			operator,
		};
	}
}

impl<'a> Executable<'a> for Operation<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		if self.operator.to_string() == "=" {
			let mut left  = execute!(engine, &self.left);
			let right = execute!(engine, &self.right).read()?;
			left.write(right)?;
			return Ok(engine.undefined());
		}

		let left  = execute!(engine, &self.left).read()?;
		let right = execute!(engine, &self.right).read()?;

		return engine.call(left.get_method(engine, &self.operator).unwrap(), vec![left, right]);
	}
}
