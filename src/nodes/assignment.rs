use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

pub struct Assignment {
    reference:  Node,
    expression: Node,
    operator:   Option<Ref<str>>,
}

impl Assignment {
    pub fn new(reference: Node, expression: Node, operator: Ref<str>) -> Self {
        Self {
            reference,
            expression,
            operator: (operator.len() > 1).then(|| Ref::new(&operator[.. operator.len() - 1]))
        }
    }
}

impl Executable for Assignment {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut reference  = get!(engine.execute(&self.reference)?);
        let mut expression = get!(engine.execute(&self.expression)?).read()?;
        if let Some(operator) = self.operator.as_ref() {
            let left = reference.read()?;
            expression = left.call_method(engine, operator, Box::new([expression]))?.read()?;
        }

        reference.write(expression)?;
        Ok(flow!(engine.undefined()))
    }
}
