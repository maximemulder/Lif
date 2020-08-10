use super::scope::Scope;
use super::classes::Classes;
use super::{ Object, Reference, Value };
use super::object::callable::Callable;

pub struct Engine<'a> {
	pub classes: Classes,
	pub scope: usize,
	references: Vec<Value>,
	objects: Vec<Object<'a>>,
	scopes: Vec<Scope>,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut engine = Self {
			classes: Classes::new(),
			scope: 0,
			references: Vec::new(),
			objects: Vec::new(),
			scopes: Vec::new(),
		};

		engine.scopes.push(Scope::new());
		engine.build_classes();

		return engine;
	}

	pub fn new_undefined(&mut self) -> Reference {
		return self.new_reference(Value::new_undefined());
	}

	pub fn new_reference(&mut self, value: Value) -> Reference {
		let reference = Reference(self.references.len());
		self.references.push(value);
		return reference;
	}

	pub fn new_value(&mut self, object: Object<'a>) -> Value {
		let value = Value(self.objects.len());
		self.objects.push(object);
		return value;
	}

	pub fn new_object(&mut self, object: Object<'a>) -> Reference {
		let value = self.new_value(object);
		return self.new_reference(value);
	}

	pub fn get_value(&mut self, reference: Reference) -> Value {
		return self.references[reference.0];
	}

	pub fn get_object(&mut self, value: Value) -> &mut Object<'a> {
		return &mut self.objects[value.0];
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

	pub fn push_frame(&mut self, frame: usize) -> usize {
		let scope = self.scope;
		self.scope = frame;
		return scope;
	}

	pub fn pop_frame(&mut self, frame: usize) {
		self.scope = frame;
	}

	pub fn new_variable(&mut self, name: &str, reference: Reference) {
		self.get_scope().add_variable(name, reference);
	}

	pub fn get_variable(&mut self, name: &str) -> Reference {
		let mut scope = self.get_scope();
		loop {
			if let Some(object) = scope.get_variable(name) {
				return object;
			}

			if let Some(parent) = scope.parent {
				scope = &mut self.scopes[parent];
			} else {
				panic!();
			}
		}
	}

	pub fn get_cast_array(&mut self, value: Value) -> &Vec<Reference> {
		self.objects[value.0].cast(self.classes.array);
		return self.objects[value.0].data.as_array();
	}

	pub fn get_cast_boolean(&mut self, value: Value) -> bool {
		self.objects[value.0].cast(self.classes.boolean);
		return *self.objects[value.0].data.as_boolean();
	}

	pub fn get_cast_callable(&mut self, value: Value) -> &dyn Callable<'a> {
		self.objects[value.0].cast(self.classes.function);
		return self.objects[value.0].data.as_callable();
	}

	pub fn read(&self, reference: Reference) -> Value {
		return self.references[reference.0];
	}

	pub fn write(&mut self, reference: Reference, value: Value) {
		self.references[reference.0] = value;
	}
}
