/* use crate::element::Element;
use crate::elements;
use crate::token::Token;
use crate::tree::Tree;

struct Node<'a, 'b, 'c, 'd> {
	pub element: &'static Element,
	pub execute: &'d[&'c dyn Fn(&mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>>],
}

fn expression<'a, 'b>(tokens: &Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	return None;
}

const A: Node = Node {
	element: &elements::SYMBOL_PLUS,
	execute: &[&expression],
};
*/