use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

use std::ops::Deref;

pub struct Sequence {
    expression:  Node,
    expressions: Box<[Node]>,
    operator:    Ref<str>,
}

impl Sequence {
    pub fn new(expression: Node, open: Ref<str>, expressions: Box<[Node]>, close: Ref<str>) -> Self {
        Self {
            expression,
            expressions,
            operator: Ref::from_ref(match format!("{}{}", open.deref(), close.deref()).as_str() {
                "()" => "__cl__",
                "[]" => "__id__",
                "<>" => "__gn__",
                _ => panic!(),
            })
        }
    }
}

impl Executable for Sequence {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let value = execute!(engine, &self.expression).read()?;
        let mut arguments = Vec::new();
        for argument in self.expressions.iter() {
            arguments.push(execute!(engine, argument));
        }

        let array = engine.new_array_value(arguments);
        value.call_method(engine, &self.operator, Box::new([array]))
    }
}
