use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::Reference;
use std::collections::HashMap;

pub struct Instance<'a> {
	pub attributes: HashMap<String, GcRef<Reference<'a>>>,
}

impl Instance<'_> {
	pub fn new() -> Self {
		return Self {
			attributes: HashMap::new(),
		};
	}
}

impl GcTraceable for Instance<'_> {
	fn trace(&mut self) {
		for attribute in self.attributes.values_mut() {
			attribute.trace();
		}
	}
}
