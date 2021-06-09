use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::runtime::utilities::variable::Variable;
use crate::walker::{ Executable, Node };
use crate::walker::utilities;

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
        let r#type = utilities::new_type(engine, self.r#type.as_ref())?;
        Flow::new(Variable::new(engine, Box::from(self.identifier.as_ref()), r#type)?.build(engine))
    }
}
