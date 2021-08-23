use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::ANode;

pub trait AStatementTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a>;
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

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        jump!(self.node.get().walk(engine)?);
        Jump::none()
    }
}
