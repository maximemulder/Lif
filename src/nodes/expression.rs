use super::Node;

pub struct Expression {
	content: Box<dyn Node>,
}

impl Node for Expression {
	fn execute(&self) {

	}
}
