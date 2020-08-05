use super::SyntaxNode;

pub struct Symbol {
	text: Box<str>,
}

impl Symbol {
	pub fn build(node: &SyntaxNode) -> Symbol {
		return Symbol {
			text: Box::from(node.text()),
		};
	}
}
