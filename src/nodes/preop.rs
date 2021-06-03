use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Flow, ReturnFlow };

use std::ops::Deref;

pub struct Preop {
    expression: Node,
    operator:   Ref<str>,
}

impl Preop {
    pub fn new(operator: Ref<str>, expression: Node) -> Self {
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

impl Executable for Preop {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let expression = engine.execute(&self.expression)?.read().map_err(Flow::Error)?;
        expression.call_method(engine, &self.operator, Box::new([])).map_err(Flow::Error)
    }
}
