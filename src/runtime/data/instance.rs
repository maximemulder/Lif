use crate::runtime::reference::Reference;
use std::collections::HashMap;

pub struct Instance<'a> {
	pub attributes: HashMap<String, Reference<'a>>,
}

impl Instance<'_> {
	pub fn new() -> Self {
		return Self {
			attributes: HashMap::new(),
		};
	}

	pub fn visit(&mut self) {
		for attribute in self.attributes.values_mut() {
			attribute.visit();
		}
	}
}
