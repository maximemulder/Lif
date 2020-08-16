use crate::runtime::Engine;
use super::{ Node, SyntaxNode, Product, Control };
use super::expression::Expression;

pub struct Break {
	expression: Option<Expression>
}

impl Break {
	pub fn build(node: &SyntaxNode) -> Self {
		return Self {
			expression: if let Some(child) = node.children().get(1) {
				Some(Expression::build(child))
			} else {
				None
			}
		};
	}
}

impl Node for Break {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product {
		return Product::new_control(if let Some(expression) = &self.expression {
			value!(expression.execute(engine))
		} else {
			engine.new_undefined()
		}, Control::Break);
	}
}
