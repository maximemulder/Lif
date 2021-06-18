use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };
use crate::walker::{ Walkable, WNode };
use crate::walker::utilities;

pub struct Return {
    expression: Option<WNode>}

impl Return {
    pub fn new(expression: Option<WNode>) -> Self {
        Self {
            expression,
        }
    }
}

impl Walkable for Return {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        utilities::new_jump(engine, Jump::Return, self.expression.as_ref())
    }
}
