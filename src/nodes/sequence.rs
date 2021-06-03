use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Flow, ReturnFlow };

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
            operator: Ref::new(match format!("{}{}", open.deref(), close.deref()).as_str() {
                "()" => "__cl__",
                "[]" => "__id__",
                "<>" => "__gn__",
                _ => panic!(),
            })
        }
    }
}

impl Executable for Sequence {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = engine.execute(&self.expression)?.read().map_err(Flow::Error)?;
        let elements = self.expressions.iter()
            .map(|expression| engine.execute(expression))
            .collect::<Result<_, _>>()?;

        let array = engine.new_array_any_value(elements);
        value.call_method(engine, &self.operator, Box::new([array])).map_err(Flow::Error)
    }
}
