use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };

impl<'a> Engine<'a> {
    pub fn jump_new(&mut self, jump: Jump, node: Option<&Node>) -> ReturnFlow<'a> {
        let reference = if let Some(node) = node {
            let value = get!(self.execute(node)?).read()?;
            self.new_constant(value)
        } else {
            self.undefined()
        };

        Ok(Flow::new(reference, jump))
    }
}
