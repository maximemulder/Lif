use crate::runtime::bis::Variable;
use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::data::{Ref, GcClass};

use std::collections::HashMap;

pub type GcObject<'a> = GcRef<Object<'a>>;

pub struct Object<'a> {
    pub attributes: HashMap<Box<str>, Variable<'a>>,
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn get_attr(&mut self, name: &str, class: GcClass<'a>) -> Ref<'a> {
        let entry = self.attributes.entry(Box::from(name));
        let variable = entry.or_insert_with(|| Variable::undefined(class));
        variable.get_ref()
    }
}

impl GcTrace for Object<'_> {
    fn trace(&mut self) {
        for attribute in self.attributes.values_mut() {
            attribute.trace();
        }
    }
}
