use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::traits::{ WLiteral, WExpression };

pub struct ALiteral {
    literal: Box<ANode<dyn WLiteral>>,
}

impl ALiteral {
	pub fn new(literal: Box<ANode<dyn WLiteral>>) -> Self {
		Self {
			literal,
		}
	}
}

impl WExpression for ALiteral {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::reference(self.literal.get().walk(engine)?)
    }
}
