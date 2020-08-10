use crate::runtime::{ Engine, Reference };
use super::expression::Expression;
use super::{ Node, SyntaxNode };

pub struct Group {
	expression: Expression,
}

impl Group {
	pub fn build(node: &SyntaxNode) -> Group {
		return Group {
			expression: Expression::build(&node.children()[node.children().len() - 1]),
		};
	}
}

impl Node for Group {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		return self.expression.execute(engine);
	}
}
