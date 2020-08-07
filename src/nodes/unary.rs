use super::expression::Expression;

pub struct Unary<'a> {
	expression: Expression,
	operator: &'a str,
}
