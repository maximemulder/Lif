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
        let elements = self.expressions.iter()
            .map(|expression| {
                let value = execute!(engine, expression).read()?;
                Ok(engine.new_reference(value))
            })
            .collect::<Result<_, _>>()?;

        Ok(engine.new_array_any(elements))
    }
}
