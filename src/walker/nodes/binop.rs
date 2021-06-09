use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

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
        let left = get!(engine.execute(&self.left)?).read()?;
        match self.operator.deref() {
            "__and__" => {
                left.cast(engine.primitives.boolean)?;
                let boolean = if *left.data_boolean() {
                    let right = get!(engine.execute(&self.right)?).read()?;
                    right.cast(engine.primitives.boolean)?;
                    *right.data_boolean()
                } else {
                    false
                };

                Flow::new(engine.new_boolean(boolean))
            },
            "__or__" => {
                left.cast(engine.primitives.boolean)?;
                let boolean = if *left.data_boolean() {
                    true
                } else {
                    let right = get!(engine.execute(&self.right)?).read()?;
                    right.cast(engine.primitives.boolean)?;
                    *right.data_boolean()
                };

                Flow::new(engine.new_boolean(boolean))
            },
            _ => {
                let right = get!(engine.execute(&self.right)?).read()?;
                Flow::new(left.call_method(engine, &self.operator, Box::new([right]))?)
            },
        }
    }
}
