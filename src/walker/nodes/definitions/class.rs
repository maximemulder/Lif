use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Class;
use crate::runtime::r#return::ReturnReference;
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ AFunction, AGenerics, AType };
use crate::walker::traits::WDefinition;

pub struct AClass {
    name: Ref<str>,
    generics: SNode<AGenerics>,
    parent: SNode<AType>,
    methods: Box<[SNode<AFunction>]>,
}

impl AClass {
    pub fn new(name: Ref<str>, generics: SNode<AGenerics>, parent: SNode<AType>, methods: Box<[SNode<AFunction>]>) -> Self {
        Self {
            name,
            generics,
            parent,
            methods,
        }
    }
}

impl ANode for AClass {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(
            node.front(1).text(),
            SNode::build(node.front(2)),
            SNode::build(node.front(3)),
            node.front(5).children().iter()
                .map(|child| SNode::build(Ref::new(child)))
                .collect(),
        )
    }
}

impl WDefinition for AClass {
    fn name(&self) -> &str {
        &self.name
    }

    fn generics(&self) -> &SNode<AGenerics> {
        &self.generics
    }

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parent = self.parent.get().walk(engine)?.unwrap_or(engine.environment.object);
        let value = engine.new_class(&self.name, Some(parent), true);
        let mut class = value.read()?.get_gc::<Class>(engine);
        engine.run_frame(class.scope(), |engine| {
            for method in self.methods.iter() {
                let function = method.get().walk(engine)?.read()?;
                class.set_method(method.get().name(), function);
            }

            Ok(())
        })?;

        Ok(value)
    }
}
