use crate::nodes::Node;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

#[derive(PartialEq, Eq)]
pub enum Jump {
    None,
    Continue,
    Break,
    Return,
}

impl<'a> Engine<'a> {
    pub fn jump_new(&mut self, jump: Jump, node: Option<&Node>) -> ReturnReference<'a> {
        let reference = if let Some(node) = node {
            let value = execute!(self, node).read()?;
            self.new_constant(value)
        } else {
            self.undefined()
        };

        self.jump_swap(Jump::None, jump);
        Ok(reference)
    }

    pub fn jump_swap(&mut self, old: Jump, new: Jump) -> bool {
        let condition = self.jump == old;
        if condition {
            self.jump = new;
        }

        condition
    }
}
