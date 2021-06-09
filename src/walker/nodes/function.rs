use crate::memory::Ref;
use crate::runtime::data::FunctionCode;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::{ Flow, Return, ReturnFlow };
use crate::runtime::utilities::variable::Variable;
use crate::walker::{ Executable, Node };
use crate::walker::utilities;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parameters = self.parameters.iter()
            .map(|(name, parameter)| {
                let r#type = utilities::new_type(engine, parameter.as_ref())?;
                Variable::new(engine, Box::from(name.as_ref()), r#type)
            })
            .collect::<Return<_>>()?;

        let rest = self.rest.as_ref().map(|(name, parameter)| {
            let r#type = parameter.as_ref().map(|parameter| {
                let r#type = engine.execute(parameter)?.none()?.read()?;
                r#type.cast(engine.primitives.class)?;
                if let Some(constructor) = r#type.data_class().constructor.as_ref() {
                    if constructor.generic != engine.primitives.array {
                        return Err(error_rest())
                    }
                } else if r#type != engine.primitives.any {
                    return Err(error_rest())
                }

                Ok(r#type)
            }).transpose()?;

            Variable::new(engine, Box::from(name.as_ref()), r#type)
        }).transpose()?;

        let r#type = utilities::new_type(engine, self.r#type.as_ref())?;
        Flow::new(engine.new_function(Ref::as_option(&self.name), parameters, rest, r#type, FunctionCode::new(Ref::new(&self.block))))
    }
}

fn error_rest() -> Error {
    Error::new_runtime("Rest parameter type must be `Any` or an array.")
}
