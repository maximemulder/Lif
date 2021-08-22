use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;

pub trait AStatementTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub struct AStatement {
    node: Box<ANode<dyn AStatementTrait>>,
}

impl AStatement {
    pub fn new(node: Box<ANode<dyn AStatementTrait>>) -> Self {
        Self {
            node,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        get!(self.node.get().walk(engine)?);
        Flow::new(engine.undefined())
    }
}
