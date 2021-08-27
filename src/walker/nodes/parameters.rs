use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::Return;
use crate::runtime::utilities::parameters::Parameters;
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::ADeclaration;


pub struct AParameters {
    parameters: Box<[SNode<ADeclaration>]>,
    rest: Option<SNode<ADeclaration>>,
}

impl AParameters {
    pub fn new(parameters: Box<[SNode<ADeclaration>]>, rest: Option<SNode<ADeclaration>>) -> Self {
        Self {
            parameters,
            rest,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> Return<Parameters<'a>> {
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

        Ok(Parameters::new(parameters, rest))
    }
}

impl ANode for AParameters {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(
            node.front(1).children().iter()
                .step_by(2)
                .map(|child| SNode::build(Ref::new(child)))
                .collect(),
            node.back(2).children().get(1).map(|child| SNode::build(Ref::new(child))),
        )
    }
}

fn error_rest() -> Error {
    Error::new_runtime("Rest parameter type must be `Any` or an array.")
}
