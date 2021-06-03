use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Flow, ReturnFlow };

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
            operator: if operator.len() > 1 {
                Some(Ref::new(&operator[.. operator.len() - 1]))
            } else {
                None
            },
        }
    }
}

impl Executable for Assignment {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut reference  = engine.execute(&self.reference)?;
        let mut expression = engine.execute(&self.expression)?.read().map_err(Flow::Error)?;
        if let Some(operator) = &self.operator {
            let left = reference.read().map_err(Flow::Error)?;
            expression = left.call_method(engine, operator, Box::new([expression])).map_err(Flow::Error)?.read().map_err(Flow::Error)?;
        }

        reference.write(expression).map_err(Flow::Error)?;
        Ok(engine.undefined())
    }
}
