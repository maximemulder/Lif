use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Structure {
    structure: Node,
}

impl Structure {
    pub fn new(structure: Node) -> Self {
        Self {
            structure
        }
    }
}

impl Executable for Structure {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let structure = execute!(engine, Ref::from_ref(&self.structure));
        engine.add_variable(structure.read()?.data_tag().get_name().unwrap(), structure);
        Ok(engine.undefined())
    }
}
