use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Integer {
    integer: isize,
}

impl Integer {
    pub fn new(string: Ref<str>) -> Self {
        let integer = if string.len() > 2 {
            match string.chars().nth(1).unwrap() {
                'b' => isize::from_str_radix(&string[2..], 2).unwrap(),
                'o' => isize::from_str_radix(&string[2..], 8).unwrap(),
                'x' => isize::from_str_radix(&string[2..], 16).unwrap(),
                _   => string.parse::<isize>().unwrap(),
            }
        } else {
            string.parse::<isize>().unwrap()
        };

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
