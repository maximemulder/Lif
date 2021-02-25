use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
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
        let mut names = Vec::new();
        for (name, parameter) in self.parameters.iter() {
            names.push(*name);
            parameters.push(if let Some(parameter) = parameter.as_ref() {
                engine.execute(parameter)?.read()?
            } else {
                engine.primitives.any
            })
        }

        let rest = if let Some(rest) = self.rest.as_ref() {
            names.push(rest.0);
            if let Some(r#type) = rest.1.as_ref() {
                Some(engine.execute(r#type)?.read()?)
            } else {
                None
            }
        } else {
            None
        };

        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(engine.execute(r#type)?.read()?)
        } else {
            None
        };

        Ok(engine.new_function(Ref::as_option(&self.name), parameters.into_boxed_slice(), rest, names.into_boxed_slice(), r#type, Ref::from_ref(&self.block)))
    }
}
