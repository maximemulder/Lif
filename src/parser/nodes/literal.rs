use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

const ELEMENTS: [&Element; 3] = [&elements::STRING, &elements::NUMBER, &elements::IDENTIFIER];

pub fn literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(token) = parser.shift() {
		for element in ELEMENTS.iter() {
			if &token.element == element {
				return Some(Node::new_production(&elements::PRODUCTION_LITERAL, vec![token]));
			}
		}

		parser.back();
	}

	return None;
}
