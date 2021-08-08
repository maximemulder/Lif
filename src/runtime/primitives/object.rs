use crate::runtime::data::PrimitiveClass;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::Class;
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

impl<'a> PrimitiveClass<'a> for Object<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.environment.object
    }
}

impl GcTrace for Object<'_> {
    fn trace(&mut self) {
        for attribute in self.attributes.values_mut() {
            attribute.trace();
        }
    }
}
