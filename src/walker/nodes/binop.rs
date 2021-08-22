use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AExpressionTrait };

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

impl AExpressionTrait for ABinop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let left = get!(self.left.get().walk(engine)?).read()?;
        match self.operator.deref() {
            "__and__" => {
                let boolean = if left.get_cast_boolean(engine)? {
                    get!(self.right.get().walk(engine)?).read()?.get_cast_boolean(engine)?
                } else {
                    false
                };

                Flow::new(engine.new_boolean(boolean))
            },
            "__or__" => {
                let boolean = if left.get_cast_boolean(engine)? {
                    true
                } else {
                    get!(self.right.get().walk(engine)?).read()?.get_cast_boolean(engine)?
                };

                Flow::new(engine.new_boolean(boolean))
            },
            _ => {
                let right = get!(self.right.get().walk(engine)?).read()?;
                Flow::new(left.call_method(engine, &self.operator, &mut [right])?)
            },
        }
    }
}
