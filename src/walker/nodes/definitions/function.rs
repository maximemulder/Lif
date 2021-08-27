use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::FunctionCode;
use crate::runtime::r#return::ReturnReference;
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ABlock, AGenerics, AParameters, AType };
use crate::walker::traits::WDefinition;

pub struct AFunction {
    name: Ref<str>,
    generics: SNode<AGenerics>,
    parameters: SNode<AParameters>,
    r#return: SNode<AType>,
    block: SNode<ABlock>,
}

impl AFunction {
    pub fn new(name: Ref<str>, generics: SNode<AGenerics>, parameters: SNode<AParameters>, r#return: SNode<AType>, block: SNode<ABlock>) -> Self {
        Self {
            name,
            generics,
            parameters,
            r#return,
            block,
        }
    }
}

impl ANode for AFunction {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(
            node.front(1).text(),
            SNode::build(node.front(2)),
            SNode::build(node.front(3)),
            SNode::build(node.front(4)),
            SNode::build(node.front(5))
        )
    }
}

impl WDefinition for AFunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn generics(&self) -> &SNode<AGenerics> {
        &self.generics
    }

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parameters = self.parameters.get().walk(engine)?;
        let r#return = self.r#return.get().walk(engine)?;
        Ok(engine.new_function(&self.name, parameters, r#return, FunctionCode::new(Ref::new(&self.block))))
    }
}
