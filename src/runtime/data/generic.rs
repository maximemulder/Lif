use crate::nodes::Executable;
use crate::runtime::data::Tag;

pub struct Generic<'a, 'b> {
    pub tag: Tag,
    pub generics: &'b [&'a str],
    pub node: &'b dyn Executable<'a>,
}

impl<'a, 'b> Generic<'a, 'b> {
    pub fn new(tag: Tag, generics: &'b [&'a str], node: &'b dyn Executable<'a>) -> Self {
        Self {
            tag,
            generics,
            node,
        }
    }
}
