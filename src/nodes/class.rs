use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Class<'a> {
    name: Option<&'a str>,
    parent: Option<Node<'a>>,
    methods: Box<[Node<'a>]>,
}

impl<'a> Class<'a> {
    pub fn new(name: Option<&'a str>, parent: Option<Node<'a>>, methods: Box<[Node<'a>]>) -> Self {
        Self {
            name,
            parent,
            methods,
        }
    }
}

impl<'a> Executable<'a> for Class<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let parent = if let Some(parent) = &self.parent {
            execute!(engine, parent).read()?
        } else {
            engine.primitives.object
        };

        let class = engine.new_class(self.name, parent);
        let mut value = class.read()?;
        let data = value.data_class_mut();
        for method in self.methods.iter() {
            let function = engine.execute(method)?.read()?;
            data.methods.insert(function.data_tag().get_name().unwrap().to_string(), function);
        }

        Ok(class)
    }
}
