use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

use std::ops::Deref;

pub struct Preop {
    expression: Node,
    operator:   Ref<str>,
}

impl Preop {
    pub fn new(operator: Ref<str>, expression: Node) -> Self {
        Self {
            expression,
            operator: Ref::from_ref(match operator.deref() {
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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let expression = execute!(engine, Ref::from_ref(&self.expression)).read()?;
        expression.get_method(&self.operator).unwrap().call(engine, vec![expression])
    }
}
