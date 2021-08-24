use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Class;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::nodes::AType;
use crate::walker::traits::WStructure;

pub struct AClass {
    name: Option<Ref<str>>,
    parent: ANode<AType>,
    methods: Box<[Box<ANode<dyn WStructure>>]>,
}

impl AClass {
    pub fn new(name: Option<Ref<str>>, parent: ANode<AType>, methods: Box<[Box<ANode<dyn WStructure>>]>) -> Self {
        Self {
            name,
            parent,
            methods,
        }
    }
}

impl WStructure for AClass {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parent = self.parent.get().walk(engine)?.unwrap_or(engine.environment.object);
        let value = engine.new_class(Ref::as_option(&self.name), Some(parent), true);
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
