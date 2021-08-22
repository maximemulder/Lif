use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AExpressionTrait };

use std::ops::Deref;

pub struct ASequence {
    expression:  ANode<AExpression>,
    expressions: Box<[ANode<AExpression>]>,
    operator:    Ref<str>,
}

impl ASequence {
    pub fn new(expression: ANode<AExpression>, open: Ref<str>, expressions: Box<[ANode<AExpression>]>, close: Ref<str>) -> Self {
        Self {
            expression,
            expressions,
            operator: Ref::new(match format!("{}{}", open.deref(), close.deref()).as_str() {
                "()" => "__cl__",
                "[]" => "__gn__",
                _ => panic!(),
            })
        }
    }
}

impl AExpressionTrait for ASequence {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = get!(self.expression.get().walk(engine)?).read()?;
        let mut elements = Vec::new();
        for expression in self.expressions.iter() {
            elements.push(get!(expression.get().walk(engine)?))
        }

        let array = engine.new_array_any_value(elements);
        Flow::new(value.call_method(engine, &self.operator, &mut [array])?)
    }
}
