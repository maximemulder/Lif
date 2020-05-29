use super::expression::Expression;

struct Binary {
	left: Box<dyn Expression>,
	right: Box<dyn Expression>,
}
