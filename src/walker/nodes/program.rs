use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Program {
    statements: WNode,
}

impl Program {
    pub fn new(statements: WNode) -> Self {
        Self {
            statements,
        }
    }
}

impl Walkable for Program {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.walk(&self.statements)?.none()?;
        Flow::new(engine.undefined())
    }
}
