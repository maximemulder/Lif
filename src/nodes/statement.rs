use super::expression::Expression;
use super::{ Engine, Node, SyntaxNode };

pub struct Statement {
	expression: Expression,
}

impl Statement {
	pub fn build(node: &SyntaxNode) -> Statement {
		return Statement {
			expression: Expression::build(&node.children()[0]),
		};
	}
}

impl Node for Statement {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Option<usize> {
		self.expression.execute(engine);
		return None;
	}
}
