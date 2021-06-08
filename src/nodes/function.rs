use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::data::FunctionCode;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::{ Return, ReturnFlow };
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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parameters = self.parameters.iter()
            .map(|(name, parameter)| {
                let r#type = parameter.as_ref().map(|parameter| get_none!(engine.execute(parameter)?).read()).transpose()?;
                Variable::new(engine, Box::from(name.as_ref()), r#type)
            })
            .collect::<Return<_>>()?;

        let rest = self.rest.as_ref().map(|(name, parameter)| {
            let r#type = parameter.as_ref().map(|parameter| {
                let r#type = get_none!(engine.execute(parameter)?).read()?;
                r#type.cast(engine.primitives.class)?;
                if let Some(constructor) = r#type.data_class().constructor.as_ref() {
                    if constructor.generic != engine.primitives.array {
                        return Err(Error::new_rest())
                    }
                } else {
                    return Err(Error::new_rest())
                }

                Ok(r#type)
            }).transpose()?;

            Variable::new(engine, Box::from(name.as_ref()), r#type)
        }).transpose()?;

        let r#type = self.r#type.as_ref().map(|r#type| get_none!(engine.execute(r#type)?).read()).transpose()?;
        Ok(flow!(engine.new_function(Ref::as_option(&self.name), parameters, rest, r#type, FunctionCode::new(Ref::new(&self.block)))))
    }
}
