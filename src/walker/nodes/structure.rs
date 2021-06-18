use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Structure {
    structure: WNode,
}

impl Structure {
    pub fn new(structure: WNode) -> Self {
        Self {
            structure
        }
    }
}

impl Walkable for Structure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let structure = get!(engine.walk(&self.structure)?);
        engine.set_variable(structure.read()?.data_tag().get_name().unwrap(), structure);
        Flow::new(engine.undefined())
    }
}
