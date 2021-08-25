use std::fmt;

pub struct Tagger {
    index: usize,
}

impl Tagger {
    pub fn new() -> Self {
        Self {
            index: 0,
        }
    }

    pub fn generate(&mut self, name: &str) -> Tag {
        let tag = Tag::new(self.index, Box::from(name));
        self.index += 1;
        tag
    }
}

#[derive(Clone)]
pub struct Tag {
    index: usize,
    name: Box<str>,
}

impl Tag {
    fn new(index: usize, name: Box<str>) -> Self {
        Self {
            index,
            name,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{}", self.name, self.index)
    }
}
