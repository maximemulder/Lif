use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
        let parent = if let Some(parent) = &self.parent {
            execute!(engine, Ref::from_ref(parent)).read()?
        } else {
            engine.primitives.object
        };

        let class = engine.new_class(self.name, parent);
        let mut value = class.read()?;
        let data = value.data_class_mut();
        for method in self.methods.iter() {
            let function = engine.execute(Ref::from_ref(method))?.read()?;
            data.methods.insert(function.data_tag().get_name().unwrap().to_string(), function);
        }

        Ok(class)
    }
}
