use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

pub struct Statements {
    statements: Box<[Node]>,
}

impl Statements {
    pub fn new(statements: Box<[Node]>) -> Self {
        Self {
            statements,
        }
    }
}

impl Executable for Statements {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        for statement in self.statements.iter() {
            get!(engine.execute(statement)?);
        }

        Flow::new(engine.undefined())
    }
}
