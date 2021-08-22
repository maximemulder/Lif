use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::nodes::ALiteralTrait;

pub struct AFloat {
    float: f64,
}

impl AFloat {
    pub fn new(float: Ref<str>) -> Self {
        let string = float.replace("_", "");
        Self {
            float: string.parse::<f64>().unwrap(),
        }
    }
}

impl ALiteralTrait for AFloat {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_float(self.float))
    }
}
