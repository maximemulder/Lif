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
		let mut left  = execute!(engine, &self.left);
		let right     = execute!(engine, &self.right);
		if self.operator.to_string() == "=" {
			*left.value_mut() = *right.value_ref();
			return engine.new_undefined();
		}

		return engine.call(*(left.value_ref().get_method(engine, &self.operator).unwrap()).value_ref(), vec![left, right]);
	}
}
