use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Structure<'a> {
    structure: Node<'a>,
}

impl<'a> Structure<'a> {
    pub fn new(structure: Node<'a>) -> Self {
        Self {
            structure
        }
    }
}

impl<'a> Executable<'a> for Structure<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let structure = execute!(engine, &self.structure);
        engine.add_variable(structure.read()?.data_tag().get_name().unwrap(), structure);
        Ok(engine.undefined())
    }
}
