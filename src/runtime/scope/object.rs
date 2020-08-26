use crate::runtime::reference::Reference;
use crate::runtime::scope::Scope;
use std::collections::HashMap;

pub struct ScopeObject<'a> {
	pub parent: Option<Scope<'a>>,
	variables: HashMap<String, Reference<'a>>,
}

impl<'a> ScopeObject<'a> {
	pub fn new() -> Self {
		return Self {
			parent: None,
			variables: HashMap::new(),
		};
	}

	pub fn new_child(scope: Scope<'a>) -> Self {
		return Self {
			parent: Some(scope),
			variables: HashMap::new(),
		};
	}

	pub fn get_variable(&self, name: &str) -> Option<Reference<'a>> {
		if let Some(reference) = self.variables.get(name) {
			return Some(*reference);
		}

		return None;
	}

	pub fn add_variable(&mut self, name: &str, reference: Reference<'a>) {
		self.variables.insert(name.to_string(), reference);
	}

	pub fn visit(&mut self) {
		if let Some(parent) = &mut self.parent {
			parent.visit();
		}

		for variable in self.variables.values_mut() {
			variable.visit();
		}
	}
}
