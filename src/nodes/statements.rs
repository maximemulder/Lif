use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statements<'a> {
    statements: Vec<Node<'a>>,
}

impl<'a> Statements<'a> {
    pub fn new(statements: Vec<Node<'a>>) -> Self {
        return Self {
            statements,
        };
    }
}

impl<'a> Executable<'a> for Statements<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        for statement in self.statements.iter() {
            execute!(engine, statement);
        }

        return Ok(engine.undefined());
    }
}
