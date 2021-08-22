mod block;
mod r#if;
mod r#loop;
mod r#while;
mod r#for;

pub use block::ABlock;
pub use r#if::AIf;
pub use r#loop::ALoop;
pub use r#while::AWhile;
pub use r#for::AFor;

use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;
use crate::walker::{ ANode, Walkable };

pub trait AControlTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub struct AControl {
	control: Box<ANode<dyn AControlTrait>>,
}

impl AControl {
	pub fn new(control: Box<ANode<dyn AControlTrait>>) -> Self {
		Self {
			control,
		}
	}
}

impl Walkable for AControl {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		self.control.get().walk(engine)
    }
}
