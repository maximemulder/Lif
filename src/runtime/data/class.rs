use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;
use std::collections::HashMap;

pub struct Class<'a, 'b> {
	pub parent:  Option<GcValue<'a, 'b>>,
	pub statics: HashMap<String, GcReference<'a, 'b>>,
	pub methods: HashMap<String, GcValue<'a, 'b>>,
}

impl<'a, 'b> Class<'a, 'b> {
	pub fn new(parent: Option<GcValue<'a, 'b>>) -> Self {
		return Self {
			parent:  parent,
			statics: HashMap::new(),
			methods: HashMap::new(),
		};
	}

	pub fn get_method(&self, engine: &Engine<'a, 'b>, name: &str) -> Option<GcValue<'a, 'b>> {
		if let Some(&method) = self.methods.get(name) {
			return Some(method);
		}

		if let Some(parent) = self.parent {
			return parent.data_class().get_method(engine, name);
		}

		return None;
	}
}

impl GcTraceable for Class<'_, '_> {
	fn trace(&mut self) {
		if let Some(parent) = &mut self.parent {
			parent.trace();
		}

		for r#static in self.statics.values_mut() {
			r#static.trace();
		}

		for method in self.methods.values_mut() {
			method.trace();
		}
	}
}
