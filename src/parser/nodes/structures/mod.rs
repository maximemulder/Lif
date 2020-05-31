mod block;
mod class;
mod conditionnals;
mod controls;
mod function;
mod loops;

use crate::node::Node;
use crate::parser::Parser;

pub fn structure<'a, 'b>(parser: &mut Parser<'a, 'b, '_>)  -> Result<Node<'a, 'b>, ()> {
	let functions: [&dyn Fn(&mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()>; 9] = [
		&block::structure_block,
		&conditionnals::structure_if,
		&controls::structure_continue,
		&controls::structure_break,
		&controls::structure_return,
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
