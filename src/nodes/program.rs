use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Program<'a> {
    statements: Node<'a>,
}

impl<'a> Program<'a> {
    pub fn new(statements: Node<'a>) -> Self {
        Self {
            statements,
        }
    }
}

impl<'a> Executable<'a> for Program<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        execute!(engine, &self.statements);
        Ok(engine.undefined())
    }
}
