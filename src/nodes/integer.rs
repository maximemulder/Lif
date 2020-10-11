use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Integer {
    integer: usize,
}

impl Integer {
    pub fn new(integer: usize) -> Self {
        return Self {
            integer,
        };
    }
}

impl<'a> Executable<'a> for Integer {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        return Ok(engine.new_integer(self.integer));
    }
}
