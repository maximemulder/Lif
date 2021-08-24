use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

use std::ops::Deref;

pub struct ABinop {
    left:     ANode<AExpression>,
    right:    ANode<AExpression>,
    operator: Ref<str>,
}

impl ABinop {
    pub fn new(left: ANode<AExpression>, operator: Ref<str>, right: ANode<AExpression>) -> Self {
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

impl WExpression for ABinop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let left = flow!(self.left.get().walk(engine)?).read()?;
        match self.operator.deref() {
            "__and__" => {
                let boolean = if left.get_cast_boolean(engine)? {
                    flow!(self.right.get().walk(engine)?).read()?.get_cast_boolean(engine)?
                } else {
                    false
                };

                Flow::reference(engine.new_boolean(boolean))
            },
            "__or__" => {
                let boolean = if left.get_cast_boolean(engine)? {
                    true
                } else {
                    flow!(self.right.get().walk(engine)?).read()?.get_cast_boolean(engine)?
                };

                Flow::reference(engine.new_boolean(boolean))
            },
            _ => {
                let right = flow!(self.right.get().walk(engine)?).read()?;
                Flow::reference(left.call_method(engine, &self.operator, &mut [right])?)
            },
        }
    }
}
