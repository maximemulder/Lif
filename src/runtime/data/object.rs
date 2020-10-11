use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use std::collections::HashMap;

pub struct Object<'a, 'b> {
    pub attributes: HashMap<String, GcReference<'a, 'b>>,
}

impl Object<'_, '_> {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }
}

impl GcTraceable for Object<'_, '_> {
    fn trace(&mut self) {
        for attribute in self.attributes.values_mut() {
            attribute.trace();
        }
    }
}
