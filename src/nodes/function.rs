use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::data::Parameter;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Function {
    name: Option<Ref<str>>,
    parameters: Box<[(Ref<str>, Option<Node>)]>,
    rest: Option<(Ref<str>, Option<Node>)>,
    r#type: Option<Node>,
    block: Node,
}

impl Function {
    pub fn new(name: Option<Ref<str>>, parameters: (Box<[(Ref<str>, Option<Node>)]>, Option<(Ref<str>, Option<Node>)>), r#type: Option<Node>, block: Node) -> Self {
        Self {
            name,
            parameters: parameters.0,
            rest: parameters.1,
            r#type,
            block,
        }
    }
}

impl Executable for Function {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut parameters = Vec::new();
        for (name, parameter) in self.parameters.iter() {
            parameters.push(Parameter::new(Box::from(name.as_ref()), if let Some(parameter) = parameter.as_ref() {
                engine.execute(parameter)?.read()?
            } else {
                engine.primitives.any
            }));
        }

        let rest = if let Some(rest) = self.rest.as_ref() {
            Some(Parameter::new(Box::from(rest.0.as_ref()), if let Some(r#type) = rest.1.as_ref() {
                engine.execute(r#type)?.read()?
            } else {
                engine.primitives.any
            }))
        } else {
            None
        };

        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(engine.execute(r#type)?.read()?)
        } else {
            None
        };

        Ok(engine.new_function(Ref::as_option(&self.name), parameters.into_boxed_slice(), rest, r#type, Ref::new(&self.block)))
    }
}
