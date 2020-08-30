use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::Reference;
use crate::runtime::value::Value;
use std::collections::HashMap;

pub struct Class<'a> {
	pub parent:  Option<GcRef<Value<'a>>>,
	pub statics: HashMap<String, GcRef<Reference<'a>>>,
	pub methods: HashMap<String, GcRef<Reference<'a>>>,
}

impl<'a> Class<'a> {
	pub fn new(parent: Option<GcRef<Value<'a>>>) -> Self {
		return Self {
			parent:  parent,
			statics: HashMap::new(),
			methods: HashMap::new(),
		}
	}

	pub fn get_method(&self, engine: &Engine<'a>, name: &str) -> Option<GcRef<Reference<'a>>> {
		if let Some(&method) = self.methods.get(name) {
			return Some(method);
		}

		if let Some(parent) = self.parent {
			return parent.data_class().get_method(engine, name);
		}

		return None;
	}
}

impl GcTraceable for Class<'_> {
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
