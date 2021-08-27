use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

pub struct AAssignment {
    reference:  SNode<AExpression>,
    expression: SNode<AExpression>,
    operator:   Option<Ref<str>>,
}

impl AAssignment {
    pub fn new(reference: SNode<AExpression>, expression: SNode<AExpression>, operator: Ref<str>) -> Self {
        Self {
            reference,
            expression,
            operator: (operator.len() > 1).then(|| Ref::new(&operator[.. operator.len() - 1]))
        }
    }
}

impl ANode for AAssignment {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(node.front(0)), SNode::build(node.front(2)), node.front(1).text())
    }
}

impl WExpression for AAssignment {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut reference  = flow!(self.reference.get().walk(engine)?);
        let mut expression = flow!(self.expression.get().walk(engine)?).read()?;
        if let Some(operator) = self.operator.as_ref() {
            let left = reference.read()?;
            expression = left.call_method(engine, operator, &mut [expression])?.read()?;
        }

        reference.write(expression)?;
        Flow::reference(engine.undefined())
    }
}
