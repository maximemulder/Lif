use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow, ReturnJump };
use crate::walker::ANode;
use crate::walker::traits::{ WStructure, WExpression, WStatement };

pub struct AStructure {
	structure: Box<ANode<dyn WStructure>>,
}

impl AStructure {
	pub fn new(structure: Box<ANode<dyn WStructure>>) -> Self {
		Self {
			structure,
		}
	}

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		self.structure.get().walk(engine)
    }
}

impl WExpression for AStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		self.walk(engine)
    }
}

impl WStatement for AStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
		Jump::flow(self.walk(engine)?)
    }
}
