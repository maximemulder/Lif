use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Integer {
    integer: isize,
}

impl Integer {
    pub fn new(integer: isize) -> Self {
        Self {
            integer,
        }
    }
}

impl Executable for Integer {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_integer(self.integer))
    }
}
