use crate::runtime::{ Value, Reference };
use std::collections::HashMap;

pub struct Class {
	parent: Option<Value>,
	statics: HashMap<String, Reference>,
	pub methods: HashMap<String, Value>,
}

impl Class {
	pub fn new() -> Self {
		return Self {
			parent:  None,
			statics: HashMap::new(),
			methods: HashMap::new(),
		}
	}
}
