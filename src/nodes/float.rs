use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;
pub struct Float {
    float: f64,
}

impl Float {
    pub fn new(float: Ref<str>) -> Self {
        let string = float.replace("_", "");
        Self {
            float: string.parse::<f64>().unwrap(),
        }
    }
}

impl Executable for Float {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_float(self.float))
    }
}
