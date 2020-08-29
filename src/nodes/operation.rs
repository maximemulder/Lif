use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference<'a> {
		if self.operator.to_string() == "=" {
			let mut left  = execute!(engine, &self.left);
			let right = execute!(engine, &self.right).value();
			*left.value_mut() = right;
			return engine.new_undefined();
		}

		let left  = execute!(engine, &self.left).value();
		let right = execute!(engine, &self.right).value();

		return engine.call((left.get_method(engine, &self.operator).unwrap()).value(), vec![left, right]);
	}
}
