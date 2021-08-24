use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow, ReturnJump };
use crate::walker::ANode;
use crate::walker::traits::{ WControl, WExpression, WStatement };

pub struct AControl {
	control: Box<ANode<dyn WControl>>,
}

impl AControl {
	pub fn new(control: Box<ANode<dyn WControl>>) -> Self {
		Self {
			control,
		}
	}

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		self.control.get().walk(engine)
    }
}

impl WExpression for AControl {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		self.walk(engine)
    }
}

impl WStatement for AControl {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
		Jump::flow(self.walk(engine)?)
    }
}
