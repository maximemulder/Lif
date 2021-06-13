use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Statements {
    statements: Box<[WNode]>,
}

impl Statements {
    pub fn new(statements: Box<[WNode]>) -> Self {
        Self {
            statements,
        }
    }
}

impl Walkable for Statements {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        for statement in self.statements.iter() {
            get!(engine.walk(statement)?);
        }

        Flow::new(engine.undefined())
    }
}
