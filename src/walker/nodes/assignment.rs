use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AExpressionTrait };

pub struct AAssignment {
    reference:  ANode<AExpression>,
    expression: ANode<AExpression>,
    operator:   Option<Ref<str>>,
}

impl AAssignment {
    pub fn new(reference: ANode<AExpression>, expression: ANode<AExpression>, operator: Ref<str>) -> Self {
        Self {
            reference,
            expression,
            operator: (operator.len() > 1).then(|| Ref::new(&operator[.. operator.len() - 1]))
        }
    }
}

impl AExpressionTrait for AAssignment {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut reference  = get!(self.reference.get().walk(engine)?);
        let mut expression = get!(self.expression.get().walk(engine)?).read()?;
        if let Some(operator) = self.operator.as_ref() {
            let left = reference.read()?;
            expression = left.call_method(engine, operator, &mut [expression])?.read()?;
        }

        reference.write(expression)?;
        Flow::new(engine.undefined())
    }
}
