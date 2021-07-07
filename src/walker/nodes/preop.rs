use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

use std::ops::Deref;

pub struct Preop {
    expression: WNode,
    operator:   Ref<str>,
}

impl Preop {
    pub fn new(operator: Ref<str>, expression: WNode) -> Self {
        Self {
            expression,
            operator: Ref::new(match operator.deref() {
                "~" => "__bnot__",
                "+" => "__pos__",
                "-" => "__neg__",
                "!" => "__not__",
                _   => panic!(),
            }),
        }
    }
}

impl Walkable for Preop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let expression = get!(engine.walk(&self.expression)?).read()?;
        Flow::new(expression.call_method(engine, &self.operator, &mut [])?)
    }
}
