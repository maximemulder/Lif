use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Result<GcReference<'a>, Error> {
		let value = execute!(engine, &self.expression).read()?;
		let mut arguments = Vec::new();
		for argument in self.expressions.iter() {
			arguments.push(execute!(engine, argument).read()?);
		}

		return engine.call(value, arguments);
	}
}
