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

    pub fn generate(&mut self, name: Option<&str>) -> Tag {
        let tag = Tag::new(self.index, name.map(Box::from));
        self.index += 1;
        tag
    }
}

#[derive(Clone)]
pub struct Tag {
    index: usize,
    name: Option<Box<str>>,
}

impl Tag {
    fn new(index: usize, name: Option<Box<str>>) -> Self {
        Self {
            index,
            name,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_ref().map(Box::as_ref)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.name.as_ref() {
            write!(f, "{}", name)?;
        }

        write!(f, "#{}", self.index)
    }
}
