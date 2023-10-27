use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::data::Ref;
use crate::runtime::bis::value::Value;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub type GcObject<'a> = GcRef<Object<'a>>;

pub struct Object<'a> {
    pub attributes: HashMap<Box<str>, Option<Value<'a>>>,
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn get_attr(&self, name: &str) -> Value<'a> {
        if let Some(Some(attribute)) = self.attributes.get(name).copied() {
            attribute
        } else {
            todo!()
        }
    }

    pub fn get_attr_ref(&mut self, name: &str) -> Ref<'a> {
        if let Entry::Occupied(mut entry) = self.attributes.entry(name.into()) {
            Ref::new(entry.get_mut())
        } else {
            self.attributes.insert(name.into(), None);
            Ref::new(self.attributes.get_mut(name).unwrap())
        }
    }
}

impl GcTrace for Object<'_> {
    fn trace(&mut self) {
        for attribute in self.attributes.values_mut() {
            if let Some(attribute) = attribute {
                attribute.trace();
            }
        }
    }
}
