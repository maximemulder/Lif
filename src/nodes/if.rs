use super::expression::Expression;
use super::Node;

pub struct If {
	condition: Expression,
	then: Expression,
	r#else: Expression,
}

impl Node for If {
	fn execute(&self) {
		self.condition.execute();
	}
}
