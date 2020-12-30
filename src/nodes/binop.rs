use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
            operator: Ref::from_ref(match operator.deref() {
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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let left = execute!(engine, Ref::from_ref(&self.left)).read()?;
        match self.operator.deref() {
            "__and__" => {
                left.cast(engine.primitives.boolean)?;
                let boolean = if *left.data_boolean() {
                    let right = execute!(engine, Ref::from_ref(&self.right)).read()?;
                    right.cast(engine.primitives.boolean)?;
                    *right.data_boolean()
                } else {
                    false
                };

                Ok(engine.new_boolean(boolean))
            },
            "__or__" => {
                left.cast(engine.primitives.boolean)?;
                let boolean = if *left.data_boolean() {
                    true
                } else {
                    let right = execute!(engine, Ref::from_ref(&self.right)).read()?;
                    right.cast(engine.primitives.boolean)?;
                    *right.data_boolean()
                };

                Ok(engine.new_boolean(boolean))
            },
            _ => {
                let right = execute!(engine, Ref::from_ref(&self.right)).read()?;
                left.get_method(&self.operator).unwrap().call(engine, vec![left, right])
            },
        }
    }
}
