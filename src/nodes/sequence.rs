use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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
        let value = get!(engine.execute(&self.expression)?).read()?;
        /* let elements = self.expressions.iter()
            .map(|expression| Ok(get_none!(engine.execute(expression)?)))
            .collect::<Return<_>>()?; */

        let mut elements = Vec::new();
        for expression in self.expressions.iter() {
            elements.push(get_none!(engine.execute(expression)?))
        }

        let array = engine.new_array_any_value(elements);
        Ok(flow!(value.call_method(engine, &self.operator, Box::new([array]))?))
    }
}
