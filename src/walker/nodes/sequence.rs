use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

use std::ops::Deref;

pub struct Sequence {
    expression:  WNode,
    expressions: Box<[WNode]>,
    operator:    Ref<str>,
}

impl Sequence {
    pub fn new(expression: WNode, open: Ref<str>, expressions: Box<[WNode]>, close: Ref<str>) -> Self {
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

impl Walkable for Sequence {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = get!(engine.walk(&self.expression)?).read()?;
        let mut elements = Vec::new();
        for expression in self.expressions.iter() {
            elements.push(get!(engine.walk(expression)?))
        }

        let array = engine.new_array_any_value(elements);
        Flow::new(value.call_method(engine, &self.operator, Box::new([array]))?)
    }
}
