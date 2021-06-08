use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let structure = get!(engine.execute(&self.structure)?);
        engine.set_variable(structure.read()?.data_tag().get_name().unwrap(), structure);
        Ok(flow!(engine.undefined()))
    }
}
