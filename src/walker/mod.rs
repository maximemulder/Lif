pub mod build;
pub mod nodes;
pub mod utilities;

use crate::memory::Ref;
use crate::parser::SNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

pub trait Walkable {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub struct WNode {
    pub syntax: Ref<SNode>,
    pub walkable: Box<dyn Walkable>,
}

impl WNode {
    pub fn new(syntax: Ref<SNode>, walkable: impl Walkable + 'static) -> Self {
        Self {
            syntax,
            walkable: Box::new(walkable),
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut flow = self.walkable.walk(engine);
        if let Err(mut error) = flow.as_mut() {
            if error.node.is_none(){
                error.node = Some(self.syntax)
            }
        }

        flow
    }
}
