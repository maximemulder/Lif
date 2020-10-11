use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Block<'a> {
    statements: Node<'a>,
    expression: Option<Node<'a>>,
}

impl<'a> Block<'a> {
    pub fn new(statements: Node<'a>, expression: Option<Node<'a>>) -> Self {
        return Self {
            statements,
            expression,
        };
    }
}

impl<'a> Executable<'a> for Block<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        engine.push_scope();
        execute!(engine, &self.statements);
        let reference = if let Some(expression) = &self.expression {
            execute!(engine, expression)
        } else {
            engine.undefined()
        };

        engine.pop_scope();
        return Ok(reference);
    }
}
