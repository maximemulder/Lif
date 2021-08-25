use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Class;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::nodes::{ AFunction, AGenerics, AType };
use crate::walker::traits::WDefinition;

pub struct AClass {
    name: Ref<str>,
    generics: ANode<AGenerics>,
    parent: ANode<AType>,
    methods: Box<[ANode<AFunction>]>,
}

impl AClass {
    pub fn new(name: Ref<str>, generics: ANode<AGenerics>, parent: ANode<AType>, methods: Box<[ANode<AFunction>]>) -> Self {
        Self {
            name,
            generics,
            parent,
            methods,
        }
    }
}

impl WDefinition for AClass {
    fn name(&self) -> &str {
        &self.name
    }

    fn generics(&self) -> &ANode<AGenerics> {
        &self.generics
    }

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parent = self.parent.get().walk(engine)?.unwrap_or(engine.environment.object);
        let value = engine.new_class(Some(&self.name), Some(parent), true);
        let mut class = value.read()?.get_gc::<Class>(engine);
        engine.run_frame(class.scope(), |engine| {
            for method in self.methods.iter() {
                let function = method.get().walk(engine)?.read()?;
                class.set_method(function.get_tag(engine).get_name().unwrap(), function);
            }

            Ok(())
        })?;

        Ok(value)
    }
}
