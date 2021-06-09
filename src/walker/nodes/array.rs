use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

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

        Flow::new(engine.new_array_any(elements))
    }
}
