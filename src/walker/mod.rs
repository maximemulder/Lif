pub mod build;
pub mod nodes;
pub mod utilities;

use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

pub use crate::node::Node as SyntaxNode;

pub trait Executable {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub struct Node {
    pub syn: Ref<SyntaxNode>,
    pub sem: Box<dyn Executable>,
}

impl Node {
    pub fn new(syn: Ref<SyntaxNode>, sem: impl Executable + 'static) -> Self {
        Self {
            syn,
            sem: Box::new(sem),
        }
    }
}

impl Executable for Node {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut flow = self.sem.execute(engine);
        if let Err(mut error) = flow.as_mut() {
            if error.node.is_none(){
                error.node = Some(self.syn)
            }
        }

        flow
    }
}
