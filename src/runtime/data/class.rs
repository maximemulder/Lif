use crate::runtime::engine::Engine;
use crate::runtime::proxy::Visitable;
use crate::runtime::reference::Reference;
use crate::runtime::value::Value;
use std::collections::HashMap;

pub struct Class<'a> {
	pub parent:  Option<Value<'a>>,
	pub statics: HashMap<String, Reference<'a>>,
	pub methods: HashMap<String, Reference<'a>>,
}

impl<'a> Class<'a> {
	pub fn new(parent: Option<Value<'a>>) -> Self {
		return Self {
			parent:  parent,
			statics: HashMap::new(),
			methods: HashMap::new(),
		}
	}

	pub fn get_method(&self, engine: &Engine<'a>, name: &str) -> Option<Reference<'a>> {
		if let Some(&method) = self.methods.get(name) {
			return Some(method);
		}

		if let Some(parent) = self.parent {
			return parent.data_class().get_method(engine, name);
		}

		return None;
	}
}

impl Visitable for Class<'_> {
	fn visit(&mut self) {
		if let Some(parent) = &mut self.parent {
			parent.visit();
		}

		for r#static in self.statics.values_mut() {
			r#static.visit();
		}

		for method in self.methods.values_mut() {
			method.visit();
		}
	}
}
