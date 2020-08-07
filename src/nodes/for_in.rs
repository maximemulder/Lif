use super::{ Engine, Node, SyntaxNode };
use super::expression::Expression;
use super::r#do::Do;
use super::identifier::Identifier;

pub struct ForIn {
	identifier: Identifier,
	expression: Expression,
	body:       Do,
}

impl ForIn {
	pub fn build(node: &SyntaxNode) -> ForIn {
		return ForIn {
			identifier: Identifier::build(&node.children()[1]),
			expression: Expression::build(&node.children()[3]),
			body:       Do::build(&node.children()[4]),
		};
	}
}

impl Node for ForIn {
	fn execute(&self, engine: &mut Engine) {
		let value = self.expression.execute(engine);
		engine.cast(value, engine.types.array);
		let elements = value.data.asArray().elements;
		for element in elements {
			engine.push_scope();
			let value = engine.new_variable(self.identifier.text.to_string());
			engine.set_variable(value);
			self.body.execute(engine);
			engine.pop_scope();
		}
	}
}
