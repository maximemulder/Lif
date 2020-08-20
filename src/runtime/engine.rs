use crate::nodes::block::Block;
use super::scope::Scope;
use super::{ Reference, Value };
use super::environment::Environment;
use super::object::data::Data;
use super::object::callable::{ Function, Primitive };
use super::object::instance::Instance;
use super::object::class::Class;
use super::object::Object;

pub struct Engine<'a> {
	pub objects: Vec<*const Value<'a>>,
	pub environment: Environment<'a>,
	pub this: Option<Reference<'a>>,
	pub scopes: Vec<Scope<'a>>,
	pub scope: usize,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut engine = Self {
			objects: Vec::new(),
			environment: Environment::new(),
			this: None,
			scopes: Vec::new(),
			scope: 0,
		};

		engine.scopes.push(Scope::new());
		engine.populate();

		return engine;
	}

	pub fn get_scope(&self) -> &Scope<'a> {
		return &self.scopes[self.scope];
	}

	pub fn get_scope_mut(&mut self) -> &mut Scope<'a> {
		return &mut self.scopes[self.scope];
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

	pub fn new_variable(&mut self, name: &str, reference: Reference<'a>) {
		self.scopes[self.scope].add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> Reference<'a> {
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

	pub fn call_method(&mut self, reference: Reference<'a>, name: &str, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
		arguments.insert(0, reference);
		return self.call(*reference.object_ref().get_method(self, name).unwrap().value_ref(), arguments);
	}

	pub fn call_method_self(&mut self, reference: Reference<'a>, name: &str, arguments: Vec<Reference<'a>>) -> Reference<'a> {
		return self.call(*reference.object_ref().get_method(self, name).unwrap().value_ref(), arguments);
	}

	pub fn call(&mut self, value: Value<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
		if let Some(this) = self.this {
			arguments.insert(0, this);
			self.this = None;
		}

		let callable = value.object_ref().data_callable().duplicate();
		return callable.call(self, arguments);
	}
}

impl<'a> Engine<'a> {
	pub fn new_array(&mut self, elements: Vec<Reference<'a>>) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.class, Data::Array(elements))));
	}

	pub fn new_boolean(&mut self, boolean: bool) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.boolean, Data::Boolean(boolean))));
	}

	pub fn new_class(&mut self) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.class, Data::Class(Class::new(Some(self.environment.object))))));
	}

	pub fn new_instance(&mut self, parent: Value<'a>) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(parent, Data::Instance(Instance::new()))));
	}

	pub fn new_integer(&mut self, integer: usize) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.integer, Data::Integer(integer))));
	}

	pub fn new_function(&mut self, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.function, Data::Callable(Box::new(Function::new(self.scope, parameters, block))))));
	}

	pub fn new_primitive(&mut self, callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference<'a>>) -> Reference<'a>) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.function, Data::Callable(Box::new(Primitive::new(callback))))));
	}

	pub fn new_string(&mut self, string: String) -> Reference<'a> {
		return Reference::new(Value::create(Object::new(self.environment.string, Data::String(string))));
	}
}
