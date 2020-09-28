use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::GcReference;
use std::collections::HashMap;

pub type GcScope<'a, 'b> = GcRef<Scope<'a, 'b>>;

pub struct Scope<'a, 'b> {
	pub parent: Option<GcScope<'a, 'b>>,
	variables: HashMap<String, GcReference<'a, 'b>>,
}

impl<'a, 'b> Scope<'a, 'b> {
	pub fn new() -> Self {
		return Self {
			parent: None,
			variables: HashMap::new(),
		};
	}

	pub fn new_child(scope: GcScope<'a, 'b>) -> Self {
		return Self {
			parent: Some(scope),
			variables: HashMap::new(),
		};
	}

	pub fn get_variable(&self, name: &str) -> Option<GcReference<'a, 'b>> {
		if let Some(reference) = self.variables.get(name) {
			return Some(*reference);
		}

		return None;
	}

	pub fn add_variable(&mut self, name: &str, reference: GcReference<'a, 'b>) {
		self.variables.insert(name.to_string(), reference);
	}
}

impl GcTraceable for Scope<'_, '_> {
	fn trace(&mut self) {
		if let Some(parent) = &mut self.parent {
			parent.trace();
		}

		for variable in self.variables.values_mut() {
			variable.trace();
		}
	}
}
