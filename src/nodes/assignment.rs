use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Assignment<'a> {
    reference:  Node<'a>,
    expression: Node<'a>,
    operator:   Option<&'a str>,
}

impl<'a> Assignment<'a> {
    pub fn new(reference: Node<'a>, expression: Node<'a>, operator: &'a str) -> Self {
        Self {
            reference,
            expression,
            operator: if operator.len() > 1 {
                Some(&operator[.. operator.len() - 1])
            } else {
                None
            },
        }
    }
}

impl<'a> Executable<'a> for Assignment<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let mut reference  = execute!(engine, &self.reference);
        let mut expression = execute!(engine, &self.expression).read()?;
        if let Some(operator) = &self.operator {
            let left = reference.read()?;
            expression = left.get_method(operator).unwrap().call(engine, vec![left, expression])?.read()?;
        }

        reference.write(expression)?;
        Ok(engine.undefined())
    }
}
