use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

use std::ops::Deref;

pub struct ABinop {
    left:     SNode<AExpression>,
    operator: Ref<str>,
    right:    SNode<AExpression>,
}

impl ABinop {
    pub fn new(left: SNode<AExpression>, operator: Ref<str>, right: SNode<AExpression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl ANode for ABinop {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(
            node.front(0)),
            Ref::new(match node.front(1).element {
                &elements::symbols::EQUAL_D        => "__eq__",
                &elements::symbols::EXCLAMATION_EQ => "__ne__",
                &elements::symbols::GUILLEMET_L    => "__lt__",
                &elements::symbols::GUILLEMET_R    => "__gt__",
                &elements::symbols::GUILLEMET_L_EQ => "__le__",
                &elements::symbols::GUILLEMET_R_EQ => "__ge__",
                &elements::symbols::AMPERSAND_D    => "__and__",
                &elements::symbols::PIPE_D         => "__or__",
                &elements::symbols::PLUS           => "__add__",
                &elements::symbols::MINUS          => "__sub__",
                &elements::symbols::ASTERISK       => "__mul__",
                &elements::symbols::SLASH          => "__div__",
                &elements::symbols::PERCENT        => "__rem__",
                &elements::symbols::AMPERSAND      => "__band__",
                &elements::symbols::PIPE           => "__bor__",
                &elements::symbols::CARET          => "__bxor__",
                &elements::symbols::GUILLEMET_L_D  => "__bls__",
                &elements::symbols::GUILLEMET_R_D  => "__brs__",
                &elements::symbols::GUILLEMET_L_T  => "__bcls__",
                &elements::symbols::GUILLEMET_R_T  => "__bcrs__",
                _ => panic!(),
            }),
            SNode::build(node.front(2)))
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
