use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Flow, ReturnFlow };

use std::ops::Deref;

pub struct Binop {
    left:     Node,
    right:    Node,
    operator: Ref<str>,
}

impl Binop {
    pub fn new(left: Node, operator: Ref<str>, right: Node) -> Self {
        Self {
            left,
            right,
            operator: Ref::new(match operator.deref() {
                "=="  => "__eq__",
                "!="  => "__ne__",
                "<"   => "__lt__",
                ">"   => "__gt__",
                "<="  => "__le__",
                ">="  => "__ge__",
                "&&"  => "__and__",
                "||"  => "__or__",
                "+"   => "__add__",
                "-"   => "__sub__",
                "*"   => "__mul__",
                "/"   => "__div__",
                "%"   => "__rem__",
                "&"   => "__band__",
                "|"   => "__bor__",
                "^"   => "__bxor__",
                "<<"  => "__bls__",
                ">>"  => "__brs__",
                "<<<" => "__bcls__",
                ">>>" => "__bcrs__",
                _     => panic!(),
            }),
        }
    }
}

impl Executable for Binop {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let left = engine.execute(&self.left)?.read().map_err(Flow::Error)?;
        match self.operator.deref() {
            "__and__" => {
                left.cast(engine.primitives.boolean).map_err(Flow::Error)?;
                let boolean = if *left.data_boolean() {
                    let right = engine.execute(&self.right)?.read().map_err(Flow::Error)?;
                    right.cast(engine.primitives.boolean).map_err(Flow::Error)?;
                    *right.data_boolean()
                } else {
                    false
                };

                Ok(engine.new_boolean(boolean))
            },
            "__or__" => {
                left.cast(engine.primitives.boolean).map_err(Flow::Error)?;
                let boolean = if *left.data_boolean() {
                    true
                } else {
                    let right = engine.execute(&self.right)?.read().map_err(Flow::Error)?;
                    right.cast(engine.primitives.boolean).map_err(Flow::Error)?;
                    *right.data_boolean()
                };

                Ok(engine.new_boolean(boolean))
            },
            _ => {
                let right = engine.execute(&self.right)?.read().map_err(Flow::Error)?;
                left.call_method(engine, &self.operator, Box::new([right])).map_err(Flow::Error)
            },
        }
    }
}
