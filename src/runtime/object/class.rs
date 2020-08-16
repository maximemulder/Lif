use crate::runtime::{ Engine, Value, Reference };
use std::collections::HashMap;

pub struct Class {
	pub parent:  Option<Value>,
	pub statics: HashMap<String, Reference>,
	pub methods: HashMap<String, Reference>,
}

impl Class {
	pub fn new(parent: Option<Value>) -> Self {
		return Self {
			parent:  parent,
			statics: HashMap::new(),
			methods: HashMap::new(),
		}
	}

	pub fn get_method(&self, engine: &Engine, name: &str) -> Option<Reference> {
		if let Some(&method) = self.methods.get(name) {
			return Some(method);
		}

		if let Some(parent) = self.parent {
			return engine.get_object(parent).data_class().get_method(engine, name);
		}

		return None;
	}
}
