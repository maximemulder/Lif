use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::traits::WLiteral;

pub struct AString {
    string: Ref<str>,
}

impl AString {
    pub fn new(string: Ref<str>) -> Self {
        Self {
            string: Ref::new(&string[1 .. string.len() - 1]),
        }
    }
}

impl WLiteral for AString {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_string(self.string.to_string()))
    }
}
