use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;
use std::collections::HashMap;

pub struct Object<'a> {
    attributes: HashMap<Box<str>, GcReference<'a>>,
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn attributes(&self) -> &HashMap<Box<str>, GcReference<'a>> {
        &self.attributes
    }

    pub fn get_attribute(&self, name: &str) -> Option<GcReference<'a>> {
        self.attributes.get(name).copied()
    }

    pub fn set_attribute(&mut self, name: &str, reference: GcReference<'a>) {
        self.attributes.insert(Box::from(name), reference);
    }
}

impl GcTrace for Object<'_> {
    fn trace(&mut self) {
        for attribute in self.attributes.values_mut() {
            attribute.trace();
        }
    }
}
