use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Flow, ReturnFlow };
use crate::runtime::utilities::variable::Variable;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(engine.execute(r#type)?.read().map_err(Flow::Error)?)
        } else {
            None
        };

        Ok(Variable::new(engine, Box::from(self.identifier.as_ref()), r#type).map_err(Flow::Error)?.build(engine))
    }
}
