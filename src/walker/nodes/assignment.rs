use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Assignment {
    reference:  WNode,
    expression: WNode,
    operator:   Option<Ref<str>>,
}

impl Assignment {
    pub fn new(reference: WNode, expression: WNode, operator: Ref<str>) -> Self {
        Self {
            reference,
            expression,
            operator: (operator.len() > 1).then(|| Ref::new(&operator[.. operator.len() - 1]))
        }
    }
}

impl Walkable for Assignment {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut reference  = get!(engine.walk(&self.reference)?);
        let mut expression = get!(engine.walk(&self.expression)?).read()?;
        if let Some(operator) = self.operator.as_ref() {
            let left = reference.read()?;
            expression = left.call_method(engine, operator, Box::new([expression]))?.read()?;
        }

        reference.write(expression)?;
        Flow::new(engine.undefined())
    }
}
