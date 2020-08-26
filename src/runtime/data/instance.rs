use crate::runtime::proxy::Visitable;
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
}

impl Visitable for Instance<'_> {
	fn visit(&mut self) {
		for attribute in self.attributes.values_mut() {
			attribute.visit();
		}
	}
}
