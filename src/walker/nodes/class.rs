use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Class as Class2;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Class {
    name: Option<Ref<str>>,
    parent: Option<WNode>,
    methods: Box<[WNode]>,
}

impl Class {
    pub fn new(name: Option<Ref<str>>, parent: Option<WNode>, methods: Box<[WNode]>) -> Self {
        Self {
            name,
            parent,
            methods,
        }
    }
}

impl Walkable for Class {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parent = if let Some(parent) = self.parent.as_ref() {
            engine.walk(parent)?.none()?.read()?.get_cast_class(engine)?
        } else {
            engine.environment.object
        };

        let value = engine.new_class(Ref::as_option(&self.name), Some(parent), true);
        let mut class = value.read()?.get_gc::<Class2>(engine);
        engine.run_frame(class.scope(), |engine| {
            for method in self.methods.iter() {
                let function = engine.walk(method)?.none()?.read()?;
                class.set_method(function.get_tag(engine).get_name().unwrap(), function);
            }

            Ok(())
        })?;

        Flow::new(value)
    }
}
