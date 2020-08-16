use crate::runtime::Engine;
use super::expression::Expression;
use super::statements::Statements;
use super::{ Node, Product };

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		engine.push_scope();
		value!(self.statements.execute(engine));
		let product = Product::new(if let Some(expression) = &self.expression {
			value!(expression.execute(engine))
		} else {
			engine.new_undefined()
		});

		engine.pop_scope();
		return product;
	}
}
