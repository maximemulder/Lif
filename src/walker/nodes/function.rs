use crate::memory::Ref;
use crate::runtime::data::FunctionCode;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::{ Flow, Return, ReturnFlow };
use crate::runtime::utilities::variable::Variable;
use crate::walker::{ Walkable, WNode };
use crate::walker::utilities;

pub struct Function {
    name: Option<Ref<str>>,
    parameters: Box<[(Ref<str>, Option<WNode>)]>,
    rest: Option<(Ref<str>, Option<WNode>)>,
    r#type: Option<WNode>,
    block: WNode,
}

impl Function {
    pub fn new(name: Option<Ref<str>>, parameters: (Box<[(Ref<str>, Option<WNode>)]>, Option<(Ref<str>, Option<WNode>)>), r#type: Option<WNode>, block: WNode) -> Self {
        Self {
            name,
            parameters: parameters.0,
            rest: parameters.1,
            r#type,
            block,
        }
    }
}

impl Walkable for Function {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parameters = self.parameters.iter()
            .map(|(name, parameter)| {
                let r#type = utilities::new_type(engine, parameter.as_ref())?;
                Variable::new(engine, Box::from(name.as_ref()), r#type)
            })
            .collect::<Return<_>>()?;

        let rest = self.rest.as_ref().map(|(name, parameter)| {
            let r#type = parameter.as_ref().map(|parameter| {
                let r#type = engine.walk(parameter)?.none()?.read()?;
                r#type.cast(engine.primitives.class)?;
                if !r#type.is_generic(engine.primitives.array) && r#type != engine.primitives.any {
                    Err(error_rest())
                } else {
                    Ok(r#type)
                }
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
