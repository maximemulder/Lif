use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

use std::ops::Deref;

pub struct ASequence {
    expression:  SNode<AExpression>,
    expressions: Box<[SNode<AExpression>]>,
    operator:    Ref<str>,
}

impl ASequence {
    pub fn new(expression: SNode<AExpression>, open: Ref<str>, expressions: Box<[SNode<AExpression>]>) -> Self {
        Self {
            expression,
            expressions,
            operator: Ref::new(match open.deref() {
                "(" => "__cl__",
                "[" => "__gn__",
                _ => panic!(),
            })
        }
    }
}

impl ANode for ASequence {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(
            SNode::build(node.front(0)),
            node.front(1).text(),
            node.front(2).children().iter()
                .step_by(2)
                .map(|child| SNode::build(Ref::new(child)))
                .collect(),
        )
    }
}

impl WExpression for ASequence {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = flow!(self.expression.get().walk(engine)?).read()?;
        let mut elements = Vec::new();
        for expression in self.expressions.iter() {
            elements.push(flow!(expression.get().walk(engine)?))
        }

        let array = engine.new_array_any_value(elements);
        Flow::reference(value.call_method(engine, &self.operator, &mut [array])?)
    }
}
