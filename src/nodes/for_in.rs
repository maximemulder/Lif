use super::expression::Expression;
use super::Node;

pub struct ForIn {
	iterable: Expression,
	body:     Expression,
}

impl Node for ForIn {
	fn execute(&self) {
		self.body.execute();
	}
}
