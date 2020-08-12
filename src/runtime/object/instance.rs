use crate::runtime::Reference;
use std::collections::HashMap;

pub struct Instance {
	pub attributes: HashMap<String, Reference>,
}

impl Instance {
	pub fn new() -> Self {
		return Self {
			attributes: HashMap::new(),
		}
	}
}
