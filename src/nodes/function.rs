use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::data::FunctionCode;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::variable::Variable;

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
            let r#type = if let Some(parameter) = parameter.as_ref() {
                Some(engine.execute(parameter)?.read()?)
            } else {
                None
            };

            parameters.push(Variable::new(engine, Box::from(name.as_ref()), r#type)?);
        }

        let rest = if let Some(rest) = self.rest.as_ref() {
            let r#type = if let Some(parameter) = rest.1.as_ref() {
                let r#type = engine.execute(parameter)?.read()?;
                r#type.cast(engine.primitives.class)?;
                if let Some(constructor) = r#type.data_class().constructor.as_ref() {
                    if constructor.generic != engine.primitives.array {
                        return Err(Error::new_rest())
                    }
                } else {
                    return Err(Error::new_rest())
                }

                Some(r#type)
            } else {
                None
            };

            Some(Variable::new(engine, Box::from(rest.0.as_ref()), r#type)?)
        } else {
            None
        };

        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(engine.execute(r#type)?.read()?)
        } else {
            None
        };

        Ok(engine.new_function(Ref::as_option(&self.name), parameters.into_boxed_slice(), rest, r#type, FunctionCode::new(Ref::new(&self.block))))
    }
}
