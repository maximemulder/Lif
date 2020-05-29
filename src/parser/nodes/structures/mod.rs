mod block;
mod conditionnals;
mod loops;

use crate::node::Node;
use crate::parser::Parser;

pub fn structure<'a, 'b>(parser: &mut Parser<'a, 'b, '_>)  -> Result<Node<'a, 'b>, ()> {
	let functions: [&dyn Fn(&mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()>; 6] = [
		&block::structure_block,
		&conditionnals::structure_if,
		&loops::structure_loop,
		&loops::structure_while,
		&loops::structure_do_while,
		&loops::structure_for_in,
	];

	for function in functions.iter() {
		if let Ok(node) = parser.safe(&|parser| function(parser)) {
			return Ok(node);
		}
	}

	return Err(());
}
