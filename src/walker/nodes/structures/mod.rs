mod function;
mod class;
mod generic;

pub use function::AFunction;
pub use class::AClass;
pub use generic::AGeneric;

use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump, ReturnReference };
use crate::walker::ANode;
use crate::walker::nodes::AStatementTrait;

pub trait AStructureTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}

pub struct AStructure {
    structure: Box<ANode<dyn AStructureTrait>>,
}

impl AStructure {
    pub fn new(structure: Box<ANode<dyn AStructureTrait>>) -> Self {
        Self {
            structure
        }
    }
}

impl AStatementTrait for AStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        let structure = self.structure.get().walk(engine)?;
        engine.set_variable(structure.read()?.get_tag(engine).get_name().unwrap(), structure);
        Jump::none()
    }
}
