use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::FunctionCode;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, AGenerics, AParameters, AType };
use crate::walker::traits::WDefinition;

pub struct AFunction {
    name: Ref<str>,
    generics: ANode<AGenerics>,
    parameters: ANode<AParameters>,
    r#return: ANode<AType>,
    block: ANode<ABlock>,
}

impl AFunction {
    pub fn new(name: Ref<str>, generics: ANode<AGenerics>, parameters: ANode<AParameters>, r#return: ANode<AType>, block: ANode<ABlock>) -> Self {
        Self {
            name,
            generics,
            parameters,
            r#return,
            block,
        }
    }
}

impl WDefinition for AFunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn generics(&self) -> &ANode<AGenerics> {
        &self.generics
    }

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parameters = self.parameters.get().walk(engine)?;
        let r#return = self.r#return.get().walk(engine)?;
        Ok(engine.new_function(&self.name, parameters, r#return, FunctionCode::new(Ref::new(&self.block))))
    }
}
