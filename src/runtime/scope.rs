use std::collections::HashMap;

pub struct Scope {
	pub parent: Option<usize>,
	variables: HashMap<String, usize>,
}

impl Scope {
	pub fn new() -> Self {
		return Self {
			parent: None,
			variables: HashMap::new(),
		};
	}

	pub fn new_child(parent: usize) -> Self {
		return Self {
			parent: Some(parent),
			variables: HashMap::new(),
		};
	}

	pub fn get_variable(&self, name: &str) -> Option<usize> {
		if let Some(value) = self.variables.get(name) {
			return Some(value.clone());
		}

		return None;
	}

	pub fn add_variable(&mut self, name: &str, value: usize) {
		self.variables.insert(name.to_string(), value);
	}
}
