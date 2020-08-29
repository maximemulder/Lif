use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::Reference;

pub struct Sequence {
	expression:  Expression,
	open:        Box<str>,
	expressions: Vec<Expression>,
	close:       Box<str>,
}

impl Sequence {
	pub fn new(expression:  Expression, open: Box<str>, expressions: Vec<Expression>, close: Box<str>) -> Self {
		return Self {
			expression,
			open,
			expressions,
			close,
		};
	}
}

impl Node for Sequence {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference<'a> {
		let reference = execute!(engine, &self.expression);
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(execute!(engine, argument));
		}

		return engine.call(*reference.value_ref(), arguments);
	}
}
