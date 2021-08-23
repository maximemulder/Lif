use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::nodes::ALiteralTrait;

use std::ops::Deref;

pub struct ABoolean {
    boolean: bool,
}

impl ABoolean {
    pub fn new(boolean: Ref<str>) -> Self {
        Self {
            boolean: match boolean.deref() {
                "true"  => true,
                "false" => false,
                _ => panic!(),
            }
        }
    }
}

impl ALiteralTrait for ABoolean {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_boolean(self.boolean))
    }
}
