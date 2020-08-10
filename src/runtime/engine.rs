use super::scope::Scope;
use super::classes::Classes;
use super::Value;

pub struct Engine<'a> {
	pub classes: Classes,
	pub scope: usize,
	values: Vec<Value<'a>>,
	scopes: Vec<Scope>,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut engine = Self {
			values: Vec::new(),
			scope: 0,
			classes: Classes::new(),
			scopes: Vec::new(),
		};

		engine.scopes.push(Scope::new());
		engine.build_classes();

		return engine;
	}

	pub fn new_undefined(&mut self) -> usize {
		let index = self.values.len();
		self.values.push(Value::new_undefined());
		return index;
	}

	pub fn new_value(&mut self, value: Value<'a>) -> usize {
		let index = self.values.len();
		self.values.push(value);
		return index;
	}

	pub fn set_value(&mut self, index: usize, value: Value<'a>) {
		self.values[index] = value;
	}

	pub fn get_value(&mut self, index: usize) -> &mut Value<'a> {
		return &mut self.values[index];
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

	pub fn new_variable(&mut self, name: &str, index: usize) -> usize {
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

	pub fn get_cast_array(&mut self, index: usize) -> Vec<usize> {
		self.values[index].cast(self.classes.array);
		return self.values[index].data.as_array().clone();
	}

	pub fn get_cast_boolean(&mut self, index: usize) -> bool {
		self.values[index].cast(self.classes.boolean);
		return *self.values[index].data.as_boolean();
	}
}
