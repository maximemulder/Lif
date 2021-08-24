use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::ANode;
use crate::walker::traits::{ WStatement, WStructure };

pub struct AStructure {
    structure: Box<ANode<dyn WStructure>>,
}

impl AStructure {
    pub fn new(structure: Box<ANode<dyn WStructure>>) -> Self {
        Self {
            structure
        }
    }
}

impl WStatement for AStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        let structure = self.structure.get().walk(engine)?;
        engine.set_variable(structure.read()?.get_tag(engine).get_name().unwrap(), structure);
        Jump::none()
    }
}
