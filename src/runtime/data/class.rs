use crate::runtime::engine::Engine;
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
			return parent.object_ref().data_class().get_method(engine, name);
		}

		return None;
	}
}
