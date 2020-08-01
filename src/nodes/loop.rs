use super::expression::Expression;
use super::Node;

pub struct Loop {
	body: Expression,
}

impl Node for Loop {
	fn execute(&self) {
		loop {
			self.body.execute();
		}
	}
}
