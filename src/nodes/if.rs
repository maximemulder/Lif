use super::expression::Expression;
use super::then::Then;
use super::r#else::Else;
use super::{ Node, SyntaxNode };

pub struct If {
	condition: Expression,
	then:      Then,
	r#else:    Option<Else>,
}

impl If {
	pub fn build(node: &SyntaxNode) -> If {
		return If {
			condition: Expression::build(&node.children()[0]),
			then:      Then::build(&node.children()[1]),
			r#else: if let Some(r#else) = node.children().get(2) {
				Some(Else::build(&r#else))
			} else {
				None
			},
		};
	}
}

impl Node for If {
	fn execute(&self) {
		self.condition.execute();
	}
}
