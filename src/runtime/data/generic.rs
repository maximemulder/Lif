use crate::nodes::Executable;

pub struct Generic<'a, 'b> {
    pub generics: &'b Box<[&'a str]>,
    pub node: &'b dyn Executable<'a>,
}

impl<'a, 'b> Generic<'a, 'b> {
    pub fn new(generics: &'b Box<[&'a str]>, node: &'b dyn Executable<'a>) -> Self {
        return Self {
            generics,
            node,
        };
    }
}
