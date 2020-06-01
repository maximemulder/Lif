use super::expression::Expression;
use super::Node;

pub struct Binary<'a> {
	left: Expression,
	right: Expression,
	operator: &'a str,
}

impl Node for Binary<'_> {
	fn execute(&self) {

	}
}
