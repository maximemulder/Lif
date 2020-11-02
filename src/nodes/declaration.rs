use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Declaration<'a> {
    identifier: &'a str,
    r#type: Option<Node<'a>>,
}

impl<'a> Declaration<'a> {
    pub fn new(identifier: &'a str, r#type: Option<Node<'a>>) -> Self {
        Self {
            identifier,
            r#type,
        }
    }
}

impl<'a> Executable<'a> for Declaration<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let r#type = if let Some(r#type) = &self.r#type {
            let value = execute!(engine, r#type).read()?;
            value.cast(engine.primitives.class)?;
            value
        } else {
            engine.primitives.any
        };

        let reference = engine.new_variable(None, r#type);
        engine.add_variable(&self.identifier, reference);
        Ok(reference)
    }
}
