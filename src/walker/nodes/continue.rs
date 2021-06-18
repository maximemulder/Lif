use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };
use crate::walker::{ Walkable, WNode };
use crate::walker::utilities;

pub struct Continue {
    expression: Option<WNode>,
}

impl Continue {
    pub fn new(expression: Option<WNode>) -> Self {
        Self {
            expression,
        }
    }
}

impl Walkable for Continue {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        utilities::new_jump(engine, Jump::Continue, self.expression.as_ref())
    }
}
