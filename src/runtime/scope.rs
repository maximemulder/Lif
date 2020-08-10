use crate::runtime::Reference;
use std::collections::HashMap;

pub struct Scope {
	pub parent: Option<usize>,
	variables: HashMap<String, Reference>,
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

	pub fn get_variable(&self, name: &str) -> Option<Reference> {
		if let Some(reference) = self.variables.get(name) {
			return Some(*reference);
		}

		return None;
	}

	pub fn add_variable(&mut self, name: &str, reference: Reference) {
		self.variables.insert(name.to_string(), reference);
	}
}
