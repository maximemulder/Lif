use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::FunctionCode;
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::utilities::parameters::Parameters;
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, ADeclaration, AType };
use crate::walker::traits::WDefinition;

pub struct AFunction {
    name: Option<Ref<str>>,
    parameters: Box<[ANode<ADeclaration>]>,
    rest: Option<ANode<ADeclaration>>,
    r#return: ANode<AType>,
    block: ANode<ABlock>,
}

impl AFunction {
    pub fn new(name: Option<Ref<str>>, parameters: (Box<[ANode<ADeclaration>]>, Option<ANode<ADeclaration>>), r#return: ANode<AType>, block: ANode<ABlock>) -> Self {
        Self {
            name,
            parameters: parameters.0,
            rest: parameters.1,
            r#return,
            block,
        }
    }
}

impl WDefinition for AFunction {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parameters = self.parameters.iter()
            .map(|parameter| parameter.get().walk(engine))
            .collect::<Return<_>>()?;

        let rest = self.rest.as_ref().map(|parameter| {
            let variable = parameter.get().walk(engine)?;
            if let Some(r#type) = variable.r#type {
                if !r#type.is_generic(engine.environment.array) && !r#type.is(engine.environment.any) {
                    return Err(error_rest());
                }
            }

            Ok(variable)
        }).transpose()?;

        let r#return = self.r#return.get().walk(engine)?;
        Ok(engine.new_function(Ref::as_option(&self.name), Parameters::new(parameters, rest), r#return, FunctionCode::new(Ref::new(&self.block))))
    }
}

fn error_rest() -> Error {
    Error::new_runtime("Rest parameter type must be `Any` or an array.")
}
