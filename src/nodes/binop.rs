use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ flow, ReturnFlow };

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
        let left = flow(engine.execute(&self.left)?.read())?;
        match self.operator.deref() {
            "__and__" => {
                flow(left.cast(engine.primitives.boolean))?;
                let boolean = if *left.data_boolean() {
                    let right = flow(engine.execute(&self.right)?.read())?;
                    flow(right.cast(engine.primitives.boolean))?;
                    *right.data_boolean()
                } else {
                    false
                };

                Ok(engine.new_boolean(boolean))
            },
            "__or__" => {
                flow(left.cast(engine.primitives.boolean))?;
                let boolean = if *left.data_boolean() {
                    true
                } else {
                    let right = flow(engine.execute(&self.right)?.read())?;
                    flow(right.cast(engine.primitives.boolean))?;
                    *right.data_boolean()
                };

                Ok(engine.new_boolean(boolean))
            },
            _ => {
                let right = flow(engine.execute(&self.right)?.read())?;
                flow(left.call_method(engine, &self.operator, Box::new([right])))
            },
        }
    }
}
