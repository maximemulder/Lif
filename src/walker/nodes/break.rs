use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };
use crate::walker::{ Walkable, WNode };
use crate::walker::utilities;

pub struct Break {
    expression: Option<WNode>,
}

impl Break {
    pub fn new(expression: Option<WNode>) -> Self {
        Self {
            expression,
        }
    }
}

impl Walkable for Break {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        utilities::new_jump(engine, Jump::Break, self.expression.as_ref())
    }
}
