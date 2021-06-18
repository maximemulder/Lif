use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::Walkable;

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

impl Walkable for Float {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.new_float(self.float))
    }
}
