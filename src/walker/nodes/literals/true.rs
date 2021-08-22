use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::nodes::ALiteralTrait;

pub struct ATrue;

impl ATrue {
    pub fn new() -> Self {
        Self
    }

}

impl ALiteralTrait for ATrue {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_boolean(true))
    }
}
