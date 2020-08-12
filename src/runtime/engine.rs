use super::scope::Scope;
use super::classes::Classes;
use super::{ Object, Reference, Value };
use super::object::callable::Callable;

pub fn cheat<T>(reference: &T) -> &mut T {
	return unsafe { (reference as *const T as *mut T).as_mut().unwrap() };
}

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

	fn get_value(&self, reference: Reference) -> &mut Value {
		return &mut cheat(self).references[reference.0];
	}

	pub fn new_undefined(&self) -> Reference {
		return cheat(self).new_reference(Value::new_undefined());
	}

	pub fn new_reference(&self, value: Value) -> Reference {
		let reference = Reference(self.references.len());
		cheat(self).references.push(value);
		return reference;
	}

	pub fn new_value(&self, object: Object<'a>) -> Value {
		let value = Value(self.objects.len());
		cheat(self).objects.push(object);
		return value;
	}

	pub fn new_object(&self, object: Object<'a>) -> Reference {
		let value = self.new_value(object);
		return self.new_reference(value);
	}

	pub fn get_object(&self, value: Value) -> &mut Object<'a> {
		return &mut cheat(self).objects[value.0];
	}

	pub fn get_scope(&self) -> &mut Scope {
		return &mut cheat(self).scopes[self.scope];
	}

	pub fn push_scope(&self) {
		cheat(self).scopes.push(Scope::new_child(self.scope));
	}

	pub fn pop_scope(&self) {
		if let Some(parent) = self.get_scope().parent {
			cheat(self).scope = parent;
		} else {
			panic!();
		}
	}

	pub fn push_frame(&self, frame: usize) -> usize {
		let scope = self.scope;
		cheat(self).scope = frame;
		return scope;
	}

	pub fn pop_frame(&self, frame: usize) {
		cheat(self).scope = frame;
	}

	pub fn new_variable(&self, name: &str, reference: Reference) {
		self.get_scope().add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> Reference {
		let mut scope = self.get_scope();
		loop {
			if let Some(object) = scope.get_variable(name) {
				return object;
			}

			if let Some(parent) = scope.parent {
				scope = &mut cheat(self).scopes[parent];
			} else {
				panic!();
			}
		}
	}

	pub fn get_cast_array(&self, value: Value) -> &Vec<Reference> {
		let object = self.get_object(value);
		object.cast(self.classes.array);
		return object.data.as_array();
	}

	pub fn get_cast_boolean(&self, value: Value) -> &bool {
		let object = self.get_object(value);
		object.cast(self.classes.boolean);
		return object.data.as_boolean();
	}

	pub fn get_cast_callable(&self, value: Value) -> &Box<dyn Callable<'a> + 'a> {
		let object = self.get_object(value);
		object.cast(self.classes.function);
		return object.data.as_callable();
	}

	pub fn read(&self, reference: Reference) -> Value {
		let value = *self.get_value(reference);
		if value == Value::new_undefined() {
			panic!();
		}

		return value;
	}

	pub fn write(&self, reference: Reference, value: Value) {
		*self.get_value(reference) = value;
	}
}
