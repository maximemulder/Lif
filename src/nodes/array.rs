use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        for expression in self.expressions.iter() {
            let value = get!(engine.execute(expression)?).read()?;
            elements.push(engine.new_reference(value))
        }

        Ok(flow!(engine.new_array_any(elements)))
    }
}
