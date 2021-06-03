use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Control, Flow, Jump, ReturnFlow };

impl<'a> Engine<'a> {
    pub fn jump_new(&mut self, control: Control, node: Option<&Node>) -> ReturnFlow<'a> {
        let reference = if let Some(node) = node {
            let value = self.execute(node)?.read().map_err(Flow::Error)?;
            self.new_constant(value)
        } else {
            self.undefined()
        };

        Err(Flow::Jump(Jump::new(control, reference)))
    }
}
