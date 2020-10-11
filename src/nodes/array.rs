use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Array<'a> {
    expressions: Box<[Node<'a>]>,
}

impl<'a> Array<'a> {
    pub fn new(expressions: Box<[Node<'a>]>) -> Self {
        Self {
            expressions,
        }
    }
}

impl<'a> Executable<'a> for Array<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let mut references = Vec::new();
        for expression in self.expressions.iter() {
            let value = execute!(engine, expression).read()?;
            references.push(engine.new_reference(value));
        }

        Ok(engine.new_array(references))
    }
}
