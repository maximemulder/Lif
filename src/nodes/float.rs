use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.new_float(self.float))
    }
}
