use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ flow, ReturnFlow };

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
        let structure = engine.execute(&self.structure)?;
        engine.set_variable(flow(structure.read())?.data_tag().get_name().unwrap(), structure);
        Ok(engine.undefined())
    }
}
