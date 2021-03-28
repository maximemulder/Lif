use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;
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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(execute!(engine, r#type).read()?)
        } else {
            None
        };

        let variable = Variable::new(engine, Box::from(self.identifier.as_ref()), r#type)?;
        Ok(variable.build(engine))
    }
}
