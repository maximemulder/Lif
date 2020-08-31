use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct Operation {
	left:     Expression,
	right:    Expression,
	operator: Box<str>,
}

impl Operation {
	pub fn new(left: Expression, right: Expression, operator: Box<str>) -> Self {
		return Self {
			left,
			right,
			operator,
		};
	}
}

impl Node for Operation {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		if self.operator.to_string() == "=" {
			let mut left  = execute!(engine, &self.left);
			let right = execute!(engine, &self.right).read();
			left.write(right);
			return engine.new_undefined();
		}

		let left  = execute!(engine, &self.left).read();
		let right = execute!(engine, &self.right).read();

		return engine.call((left.get_method(engine, &self.operator).unwrap()).read(), vec![left, right]);
	}
}
