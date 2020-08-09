use std::collections::HashMap;

pub struct Class {
	parent: Option<usize>,
	statics: HashMap<String, usize>,
	methods: HashMap<String, usize>,
}

impl Class {
	pub fn new() -> Self {
		return Self {
			parent: None,
			statics: HashMap::new(),
			methods: HashMap::new(),
		}
	}
}
