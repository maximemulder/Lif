use super::{ Engine, Node, SyntaxNode };
use crate::elements;
use super::block::Block;
use super::r#if::If;
use super::r#loop::Loop;
use super::r#while::While;
use super::for_in::ForIn;

pub struct Structure {
	content: Box<dyn Node>,
}

impl Structure {
	pub fn build(node: &SyntaxNode) -> Structure {
		let child = &node.children()[0];
		return Structure {
			content: match child.element {
				&elements::structures::BLOCK  => Box::new(Block::build(child)),
				&elements::structures::IF     => Box::new(If::build(child)),
				&elements::structures::LOOP   => Box::new(Loop::build(child)),
				&elements::structures::WHILE  => Box::new(While::build(child)),
				&elements::structures::FOR_IN => Box::new(ForIn::build(child)),
				_ => panic!(),
			},
		};
	}
}

impl Node for Structure {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Option<usize> {
		return self.content.execute(engine);
	}
}
