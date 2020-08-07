use super::expression::Expression;
use super::expressions::Expressions;
use super::symbol::Symbol;
use super::{ Engine, Node, SyntaxNode };

pub struct Sequence {
	expression:  Expression,
	open:        Symbol,
	expressions: Expressions,
	close:       Symbol,
}

impl Sequence {
	pub fn build(node: &SyntaxNode) -> Sequence {
		return Sequence {
			expression: Expression::build(&node.children()[0]),
			open:       Symbol::build(&node.children()[1]),
			expressions: Expressions::build(&node.children()[2]),
			close:      Symbol::build(&node.children()[3]),
		};
	}
}

impl Node for Sequence {
	fn execute(&self, engine: &mut Engine) {
		loop {
			self.expression.execute(engine);
		}
	}
}
