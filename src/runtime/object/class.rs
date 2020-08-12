use crate::runtime::{ Engine, Value, Reference };
use std::collections::HashMap;

pub struct Class {
	pub parent:      Option<Value>,
	pub statics:     HashMap<String, Reference>,
	pub methods: HashMap<String, Reference>,
}

impl Class {
	pub fn new() -> Self {
		return Self {
			parent:  None,
			statics: HashMap::new(),
			methods: HashMap::new(),
		}
	}

	pub fn get_method(&self, engine: &Engine, name: &String) -> Option<Reference> {
		if let Some(&method) = self.methods.get(name) {
			return Some(method);
		}

		if let Some(parent) = self.parent {
			return engine.get_object(parent).data.as_class().get_method(engine, name);
		}

		return None;
	}
}
