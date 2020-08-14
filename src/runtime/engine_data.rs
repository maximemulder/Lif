use crate::nodes::block::Block;
use super::scope::Scope;
use super::primitives::Primitives;
use super::{ Engine, Object, Reference, Value };
use super::object::data::Data;
use super::object::callable::{ Function, Primitive };
use super::object::instance::Instance;
use super::object::class::Class;

pub struct EngineData<'a> {
	pub primitives: Primitives,
	pub scope: usize,
	pub this: Option<Reference>,
	pub references: Vec<Value>,
	pub objects: Vec<Object<'a>>,
	pub scopes: Vec<Scope>,
}

impl<'a> EngineData<'a> {
	pub fn new() -> Self {
		return Self {
			primitives: Primitives::new(),
			scope: 0,
			references: Vec::new(),
			objects: Vec::new(),
			scopes: Vec::new(),
			this: None,
		};
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
}

impl<'a> EngineData<'a> {
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

	pub fn new_primitive(&mut self, callback: &'static dyn for<'b> Fn(&'b Engine, Vec<Reference>) -> Reference) -> Reference {
		return self.new_object(Object::new(self.primitives.function, Data::Callable(Box::new(Primitive::new(callback)))));
	}

	pub fn new_string(&mut self, string: String) -> Reference {
		return self.new_object(Object::new(self.primitives.string, Data::String(string)));
	}
}
