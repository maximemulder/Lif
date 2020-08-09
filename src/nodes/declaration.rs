use super::{ Engine, Node, SyntaxNode };
use super::token::Token;

pub struct Declaration {
	identifier: Box<str>,
}

impl Declaration {
	pub fn build(node: &SyntaxNode) -> Declaration {
		return Declaration {
			identifier: Token::build(&node.children()[1]),
		};
	}
}

impl Node for Declaration {
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		let value = engine.new_undefined();
		return Some(engine.new_variable(&self.identifier, value));
	}
}
