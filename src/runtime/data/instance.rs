use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use std::collections::HashMap;

pub struct Instance<'a, 'b> {
	pub attributes: HashMap<String, GcReference<'a, 'b>>,
}

impl Instance<'_, '_> {
	pub fn new() -> Self {
		return Self {
			attributes: HashMap::new(),
		};
	}
}

impl GcTraceable for Instance<'_, '_> {
	fn trace(&mut self) {
		for attribute in self.attributes.values_mut() {
			attribute.trace();
		}
	}
}
