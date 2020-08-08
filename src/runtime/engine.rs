use super::scope::Scope;
use super::Value;

pub struct Engine {
	values: Vec<Option<Value>>,
	scopes: Vec<Scope>,
	scope: usize,
}

impl Engine {
	pub fn new() -> Self {
		return Self {
			values: Vec::new(),
			scopes: vec![Scope::new()],
			scope: 0,
		};
	}

	pub fn new_undefined(&mut self) -> usize {
		let index = self.values.len();
		self.values.push(None);
		return index;
	}

	pub fn new_value(&mut self, value: Value) -> usize {
		let index = self.values.len();
		self.values.push(Some(value));
		return index;
	}

	pub fn set_value(&mut self, index: usize, value: Value) {
		self.values[index] = Some(value);
	}

	pub fn get_value(&mut self, index: usize) -> &Value {
		if let Some(value) = &self.values[index] {
			return value;
		}

		panic!();
	}

	pub fn get_scope(&mut self) -> &mut Scope {
		return &mut self.scopes[self.scope];
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new_child(self.scope));
	}

	pub fn pop_scope(&mut self) {
		if let Some(parent) = self.get_scope().parent {
			self.scope = parent;
		} else {
			panic!();
		}
	}

	pub fn new_variable(&mut self, name: &str) -> usize {
		let index = self.new_undefined();
		self.get_scope().add_variable(name, index);
		return index;
	}

	pub fn get_variable(&mut self, name: &str) -> usize {
		let mut scope = self.get_scope();
		loop {
			if let Some(value) = scope.get_variable(name) {
				return value;
			}

			if let Some(parent) = scope.parent {
				scope = &mut self.scopes[parent];
			} else {
				panic!();
			}
		}
	}

	/* pub fn get_variable_value(&mut self, name: &str) {
		let index = self.get_variable(name);
		self.get_value(index);
	} */
}
