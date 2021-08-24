use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::ANode;
use crate::walker::traits::WStatement;

pub struct AStatement {
    node: Box<ANode<dyn WStatement>>,
}

impl AStatement {
    pub fn new(node: Box<ANode<dyn WStatement>>) -> Self {
        Self {
            node,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        jump!(self.node.get().walk(engine)?);
        Jump::none()
    }
}
