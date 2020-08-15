use crate::nodes::block::Block;
use super::scope::Scope;
use super::{ Object, Reference, Value };
use super::object::callable::Callable;
use super::primitives::Primitives;
use super::object::data::Data;
use super::object::callable::{ Function, Primitive };
use super::object::instance::Instance;
use super::object::class::Class;

pub struct Engine<'a> {
	pub primitives: Primitives,
	pub scope: usize,
	pub this: Option<Reference>,
	pub references: Vec<Value>,
	pub objects: Vec<Object<'a>>,
	pub scopes: Vec<Scope>,
}

impl<'a> Engine<'a> {
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
		engine.populate();

		return engine;
	}

	pub fn get_value(&self, reference: Reference) -> &Value {
		return &self.references[reference.0];
	}

	pub fn get_value_mut(&mut self, reference: Reference) -> &mut Value {
		return &mut self.references[reference.0];
	}

	pub fn get_scope(&self) -> &Scope {
		return &self.scopes[self.scope];
	}

	pub fn get_scope_mut(&mut self) -> &mut Scope {
		return &mut self.scopes[self.scope];
	}

	pub fn get_object(&self, value: Value) -> &Object<'a> {
		return &self.objects[value.0];
	}

	pub fn get_object_mut(&mut self, value: Value) -> &mut Object<'a> {
		return &mut self.objects[value.0];
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

	pub fn push_scope(&mut self) {
		let index = self.scope;
		self.scopes.push(Scope::new_child(index));
		self.scope = self.scopes.len() - 1;
	}

	pub fn pop_scope(&mut self) {
		if let Some(parent) = self.scopes[self.scope].parent {
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
		self.scopes[self.scope].add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> Reference {
		let index = self.scope;
		let mut scope = &self.scopes[index];
		loop {
			if let Some(object) = scope.get_variable(name) {
				return object;
			}

			if let Some(parent) = scope.parent {
				scope = &self.scopes[parent];
			} else {
				panic!();
			}
		}
	}

	pub fn get_cast_array(&self, value: Value) -> &Vec<Reference> {
		let object = self.get_object(value);
		object.cast(self.primitives.function);
		return object.data.as_array();
	}

	pub fn get_cast_boolean(&self, value: Value) -> &bool {
		let object = self.get_object(value);
		object.cast(self.primitives.function);
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
		let value: Value = self.get_value(reference).clone();
		if value == Value::new_undefined() {
			panic!();
		}

		return value;
	}

	pub fn write(&mut self, reference: Reference, value: Value) {
		*self.get_value_mut(reference) = value;
	}

	pub fn call_method(&mut self, reference: Reference, name: &str, mut arguments: Vec<Reference>) -> Reference {
		arguments.insert(0, reference);
		return self.call(self.read(self.get_object(self.read(reference)).get_method(self, name).unwrap()), arguments);
	}

	pub fn call_method_self(&mut self, reference: Reference, name: &str, arguments: Vec<Reference>) -> Reference {
		return self.call(self.read(self.get_object(self.read(reference)).get_method(self, name).unwrap()), arguments);
	}

	pub fn call(&mut self, value: Value, mut arguments: Vec<Reference>) -> Reference {
		if let Some(this) = self.this {
			arguments.insert(0, this);
			self.this = None;
		}

		let callable = self.get_object(value).data.as_callable().duplicate();
		return callable.call(self, arguments);
	}
}

impl<'a> Engine<'a> {
	pub fn new_array(&mut self, elements: Vec<Reference>) -> Reference {
		return self.new_object(Object::new(self.primitives.class, Data::Array(elements)));
	}

	pub fn new_boolean(&mut self, boolean: bool) -> Reference {
		return self.new_object(Object::new(self.primitives.boolean, Data::Boolean(boolean)));
	}

	pub fn new_class(&mut self) -> Reference {
		return self.new_object(Object::new(self.primitives.class, Data::Class(Class::new(Some(self.primitives.object)))));
	}

	pub fn new_instance(&mut self, parent: Value) -> Reference {
		return self.new_object(Object::new(parent, Data::Instance(Instance::new())));
	}

	pub fn new_integer(&mut self, integer: usize) -> Reference {
		return self.new_object(Object::new(self.primitives.integer, Data::Integer(integer)));
	}

	pub fn new_function(&mut self, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Reference {
		return self.new_object(Object::new(self.primitives.function, Data::Callable(Box::new(Function::new(self.scope, parameters, block)))));
	}

	pub fn new_primitive(&mut self, callback: &'static dyn for<'b> Fn(&'b mut Engine, Vec<Reference>) -> Reference) -> Reference {
		return self.new_object(Object::new(self.primitives.function, Data::Callable(Box::new(Primitive::new(callback)))));
	}

	pub fn new_string(&mut self, string: String) -> Reference {
		return self.new_object(Object::new(self.primitives.string, Data::String(string)));
	}
}
