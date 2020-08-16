use super::{ Node, SyntaxNode };
use crate::elements;
use super::r#return::Return;
use super::r#break::Break;
use super::r#continue::Continue;

pub fn control(node: &SyntaxNode) -> Box<dyn Node> {
	let child = &node.children()[0];
	return match child.element {
		&elements::controls::RETURN   => Box::new(Return::build(child)),
		&elements::controls::BREAK    => Box::new(Break::build(child)),
		&elements::controls::CONTINUE => Box::new(Continue::build(child)),
		_ => panic!(),
	};
}
