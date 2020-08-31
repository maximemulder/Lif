use crate::nodes::Node;
use crate::nodes::expression::Expression;
use crate::nodes::statements::Statements;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct Block {
	statements: Statements,
	expression: Option<Expression>,
}

impl Block {
	pub fn new(statements: Statements, expression: Option<Expression>) -> Self {
		return Self {
			statements,
			expression,
		};
	}
}

impl Node for Block {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		engine.push_scope();
		execute!(engine, &self.statements);
		let reference = if let Some(expression) = &self.expression {
			execute!(engine, expression)
		} else {
			engine.new_undefined()
		};

		engine.pop_scope();
		return reference;
	}
}
