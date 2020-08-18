use crate::runtime::{ Engine, Value, Reference };
use std::collections::HashMap;

pub struct Class<'a> {
	pub parent:  Option<*mut Value<'a>>,
	pub statics: HashMap<String, Reference<'a>>,
	pub methods: HashMap<String, Reference<'a>>,
}

impl<'a> Class<'a> {
	pub fn new(parent: Option<*mut Value<'a>>) -> Self {
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
