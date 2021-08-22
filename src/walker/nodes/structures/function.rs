use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::FunctionCode;
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::utilities::variable::Variable;
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, AType, AStructureTrait };

pub struct AFunction {
    name: Option<Ref<str>>,
    parameters: Box<[(Ref<str>, ANode<AType>)]>,
    rest: Option<(Ref<str>, ANode<AType>)>,
    r#return: ANode<AType>,
    block: ANode<ABlock>,
}

impl AFunction {
    pub fn new(name: Option<Ref<str>>, parameters: (Box<[(Ref<str>, ANode<AType>)]>, Option<(Ref<str>, ANode<AType>)>), r#return: ANode<AType>, block: ANode<ABlock>) -> Self {
        Self {
            name,
            parameters: parameters.0,
            rest: parameters.1,
            r#return,
            block,
        }
    }
}

impl AStructureTrait for AFunction {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parameters = self.parameters.iter()
            .map(|(name, parameter)| {
                let r#type = parameter.get().walk(engine)?;
                Ok(Variable::new(Box::from(name.as_ref()), r#type))
            })
            .collect::<Return<_>>()?;

        let rest = self.rest.as_ref().map(|(name, parameter)| {
            let r#type = parameter.get().walk(engine)?.map(|r#type| {
                if r#type.is_generic(engine.environment.array) || r#type.is(engine.environment.any) {
                    Ok(r#type)
                } else {
                    Err(error_rest())
                }
            }).transpose()?;

            Ok(Variable::new(Box::from(name.as_ref()), r#type))
        }).transpose()?;

        let r#return = self.r#return.get().walk(engine)?;
        Ok(engine.new_function(Ref::as_option(&self.name), Parameters::new(parameters, rest), r#return, FunctionCode::new(Ref::new(&self.block))))
    }
}

fn error_rest() -> Error {
    Error::new_runtime("Rest parameter type must be `Any` or an array.")
}
