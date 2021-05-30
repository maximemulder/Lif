use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Class {
    name: Option<Ref<str>>,
    parent: Option<Node>,
    methods: Box<[Node]>,
}

impl Class {
    pub fn new(name: Option<Ref<str>>, parent: Option<Node>, methods: Box<[Node]>) -> Self {
        Self {
            name,
            parent,
            methods,
        }
    }
}

impl Executable for Class {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let parent = if let Some(parent) = self.parent.as_ref() {
            execute!(engine, parent).read()?
        } else {
            engine.primitives.object
        };

        let class = engine.new_class(Ref::as_option(&self.name), Some(parent));
        let value = class.read()?;
        engine.run_frame(value.data_class().scope(), &|engine| {
            let mut value = class.read()?;
            let data = value.data_class_mut();
            for method in self.methods.iter() {
                let function = engine.execute(method)?.read()?;
                data.methods.insert(function.data_tag().get_name().unwrap().to_string().into_boxed_str(), function);
            }

            Ok(())
        })?;

        Ok(class)
    }
}
