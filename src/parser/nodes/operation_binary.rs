use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

pub fn operation_binary<'a, 'b, 'c>(
	parser: &mut Parser<'a, 'b, 'c>,
	operators: &[&'a Element],
	expression_left:  &Next<'a, 'b, 'c>,
	expression_right: &Next<'a, 'b, 'c>,
) -> Result<Node<'a, 'b>, ()> {
	let mut expression = expression_left(parser)?;
	if let Ok(mut nodes) = parser.safes(&|parser| Ok(vec![
		parser.tokens(&operators)?,
		expression_right(parser)?,
	])) {
		nodes.insert(0, expression);
		expression = Node::new_expression(&elements::PRODUCTION_OPERATION, nodes);
	}

	return Ok(expression);
}
