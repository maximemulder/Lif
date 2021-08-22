use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::nodes::ALiteralTrait;

pub struct AFalse;

impl AFalse {
    pub fn new() -> Self {
        Self
    }
}

impl ALiteralTrait for AFalse {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_boolean(false))
    }
}
