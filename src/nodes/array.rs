use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Array {
    expressions: Box<[Node]>,
}

impl Array {
    pub fn new(expressions: Box<[Node]>) -> Self {
        Self {
            expressions,
        }
    }
}

impl Executable for Array {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut references = Vec::new();
        for expression in self.expressions.iter() {
            let value = execute!(engine, expression).read()?;
            references.push(engine.new_reference(value));
        }

        Ok(engine.new_array(references))
    }
}
