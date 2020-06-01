use super::expression::Expression;
use super::Node;

pub struct Statement {
	expression: Expression,
}

impl Node for Statement {
	fn execute(&self) {
		self.expression.execute();
	}
}
