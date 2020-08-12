use super::scope::Scope;
use super::primitives::Primitives;
use super::{ Object, Reference, Value };
use super::object::callable::Callable;

pub struct Engine<'a> {
	pub primitives: Primitives,
	pub scope: usize,
	pub this: Option<Value>,
	references: Vec<Value>,
	objects: Vec<Object<'a>>,
	scopes: Vec<Scope>,
}

impl<'a> Engine<'a> {
	pub fn cheat(&self) -> &mut Engine<'a> {
		return unsafe { (self as *const Engine<'a> as *mut Engine<'a>).as_mut().unwrap() };
	}

	pub fn new() -> Self {
		let mut engine = Self {
			primitives: Primitives::new(),
			scope: 0,
			references: Vec::new(),
			objects: Vec::new(),
			scopes: Vec::new(),
			this: None,
		};

		engine.scopes.push(Scope::new());
		engine.build_primitives();

		return engine;
	}

	fn get_value(&self, reference: Reference) -> &mut Value {
		return &mut self.cheat().references[reference.0];
	}

	pub fn new_undefined(&self) -> Reference {
		return self.cheat().new_reference(Value::new_undefined());
	}

	pub fn new_reference(&self, value: Value) -> Reference {
		let reference = Reference(self.references.len());
		self.cheat().references.push(value);
		return reference;
	}

	pub fn new_value(&self, object: Object<'a>) -> Value {
		let value = Value(self.objects.len());
		self.cheat().objects.push(object);
		return value;
	}

	pub fn new_object(&self, object: Object<'a>) -> Reference {
		let value = self.new_value(object);
		return self.new_reference(value);
	}

	pub fn get_object(&self, value: Value) -> &mut Object<'a> {
		return &mut self.cheat().objects[value.0];
	}

	pub fn get_scope(&self) -> &mut Scope {
		return &mut self.cheat().scopes[self.scope];
	}

	pub fn push_scope(&self) {
		self.cheat().scopes.push(Scope::new_child(self.scope));
	}

	pub fn pop_scope(&self) {
		if let Some(parent) = self.get_scope().parent {
			self.cheat().scope = parent;
		} else {
			panic!();
		}
	}

	pub fn push_frame(&self, frame: usize) -> usize {
		let scope = self.scope;
		self.cheat().scope = frame;
		return scope;
	}

	pub fn pop_frame(&self, frame: usize) {
		self.cheat().scope = frame;
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
				scope = &mut self.cheat().scopes[parent];
			} else {
				panic!();
			}
		}
	}

	pub fn get_cast_array(&self, value: Value) -> &Vec<Reference> {
		let object = self.get_object(value);
		object.cast(self.primitives.array);
		return object.data.as_array();
	}

	pub fn get_cast_boolean(&self, value: Value) -> &bool {
		let object = self.get_object(value);
		object.cast(self.primitives.boolean);
		return object.data.as_boolean();
	}

	pub fn get_cast_callable(&self, value: Value) -> &Box<dyn Callable<'a> + 'a> {
		let object = self.get_object(value);
		object.cast(self.primitives.function);
		return object.data.as_callable();
	}

	pub fn get_cast_string(&self, value: Value) -> &String {
		let object = self.get_object(value);
		object.cast(self.primitives.string);
		return object.data.as_string();
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
