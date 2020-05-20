use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

const ELEMENTS: [&Element; 3] = [&elements::STRING, &elements::NUMBER, &elements::IDENTIFIER];

pub fn literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	let token = parser.shift()?;
	for element in ELEMENTS.iter() {
		if &token.element == element {
			return Ok(Node::new_production(&elements::PRODUCTION_LITERAL, vec![token]));
		}
	}

	parser.back();
	return Err(());
}
