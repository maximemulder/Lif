use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Statement {
    node: WNode,
}

impl Statement {
    pub fn new(node: WNode) -> Self {
        Self {
            node,
        }
    }
}

impl Walkable for Statement {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        get!(engine.walk(&self.node)?);
        Flow::new(engine.undefined())
    }
}
