use super::expression::Expression;
use super::Node;

pub struct While {
	condition: Expression,
	body:      Expression,
}

impl Node for While {
	fn execute(&self) {
		self.condition.execute();
	}
}
