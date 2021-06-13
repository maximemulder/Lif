use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;
use crate::walker::{ Walkable, WNode };

pub struct Group {
    expression: WNode,
}

impl Group {
    pub fn new(expression: WNode) -> Self {
        Self {
            expression,
        }
    }
}

impl Walkable for Group {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.walk(&self.expression)
    }
}
