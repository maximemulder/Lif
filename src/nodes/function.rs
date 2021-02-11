use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Function {
    name: Option<Ref<str>>,
    parameters: Box<[Node]>,
    r#type: Option<Node>,
    block: Node,
}

impl Function {
    pub fn new(name: Option<Ref<str>>, parameters: Box<[Node]>, r#type: Option<Node>, block: Node) -> Self {
        Self {
            name,
            parameters,
            r#type,
            block,
        }
    }
}

impl Executable for Function {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(engine.execute(r#type)?.read()?)
        } else {
            None
        };

        Ok(engine.new_function(Ref::as_option(&self.name), Ref::from_ref(&self.parameters), r#type, Ref::from_ref(&self.block)))
    }
}
