use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::runtime::value::Primitive;
use crate::walker::{ Walkable, WNode };

use std::ops::Deref;

pub struct Binop {
    left:     WNode,
    right:    WNode,
    operator: Ref<str>,
}

impl Binop {
    pub fn new(left: WNode, operator: Ref<str>, right: WNode) -> Self {
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

impl Walkable for Binop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let left = get!(engine.walk(&self.left)?).read()?;
        match self.operator.deref() {
            "__and__" => {
                left.cast(engine, engine.primitives.boolean)?;
                let boolean = if bool::get(engine, left) {
                    let right = get!(engine.walk(&self.right)?).read()?;
                    right.cast(engine, engine.primitives.boolean)?;
                    bool::get(engine, right)
                } else {
                    false
                };

                Flow::new(engine.new_boolean(boolean))
            },
            "__or__" => {
                left.cast(engine, engine.primitives.boolean)?;
                let boolean = if bool::get(engine, left) {
                    true
                } else {
                    let right = get!(engine.walk(&self.right)?).read()?;
                    right.cast(engine, engine.primitives.boolean)?;
                    bool::get(engine, right)
                };

                Flow::new(engine.new_boolean(boolean))
            },
            _ => {
                let right = get!(engine.walk(&self.right)?).read()?;
                Flow::new(left.call_method(engine, &self.operator, &mut [right])?)
            },
        }
    }
}
