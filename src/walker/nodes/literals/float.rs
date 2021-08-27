use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::traits::WLiteral;

pub struct AFloat {
    float: f64,
}

impl AFloat {
    pub fn new(float: f64) -> Self {
        Self {
            float,
        }
    }
}

impl ANode for AFloat {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.text().replace("_", "").parse::<f64>().unwrap())
    }
}

impl WLiteral for AFloat {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_float(self.float))
    }
}
