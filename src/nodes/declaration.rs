use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Declaration {
    identifier: Ref<str>,
    r#type: Option<Node>,
}

impl Declaration {
    pub fn new(identifier: Ref<str>, r#type: Option<Node>) -> Self {
        Self {
            identifier,
            r#type,
        }
    }
}

impl Executable for Declaration {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let r#type = if let Some(r#type) = &self.r#type {
            let value = execute!(engine, Ref::from_ref(r#type)).read()?;
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
