use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Array {
    expressions: Box<[WNode]>,
}

impl Array {
    pub fn new(expressions: Box<[WNode]>) -> Self {
        Self {
            expressions,
        }
    }
}

impl Walkable for Array {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        for expression in self.expressions.iter() {
            let value = get!(engine.walk(expression)?).read()?;
            elements.push(engine.new_reference(value))
        }

        Flow::new(engine.new_array_any(elements))
    }
}
