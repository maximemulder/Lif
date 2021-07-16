use crate::memory::Ref;
use crate::runtime::data::Class as Class2;
use crate::runtime::engine::Engine;
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
            engine.walk(parent)?.none()?.read()?
        } else {
            engine.primitives.object
        };

        let class = engine.new_class(Ref::as_option(&self.name), Some(parent));
        let mut value = class.read()?;
        engine.run_frame(value.get_ref::<Class2>(engine).scope(), |engine| {
            let data = value.get_mut::<Class2>(engine);
            for method in self.methods.iter() {
                let function = engine.walk(method)?.none()?.read()?;
                data.set_method(function.data_tag().get_name().unwrap(), function);
            }

            Ok(())
        })?;

        Flow::new(class)
    }
}
