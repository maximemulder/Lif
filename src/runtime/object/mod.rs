pub mod callable;
pub mod class;
pub mod data;
pub mod instance;

use crate::runtime::{ Engine, Reference, Value };
use crate::nodes::block::Block;
use data::Data;
use class::Class;
use instance::Instance;
use callable::{ Function, Primitive };

pub struct Object<'a> {
	pub class: Value,
	pub data: Data<'a>,
}

impl<'a> Object<'a> {
	pub fn new(class: Value, data: Data<'a>) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn new_array(engine: &Engine, elements: Vec<Reference>) -> Self {
		return Self::new(engine.primitives.class, Data::Array(elements));
	}

	pub fn new_boolean(engine: &Engine, boolean: bool) -> Self {
		return Self::new(engine.primitives.boolean, Data::Boolean(boolean));
	}

	pub fn new_class(engine: &Engine) -> Self {
		return Self::new(engine.primitives.class, Data::Class(Class::new()));
	}

	pub fn new_instance(_: &Engine, parent: Value) -> Self {
		return Self::new(parent, Data::Instance(Instance::new()));
	}

	pub fn new_integer(engine: &Engine, integer: usize) -> Self {
		return Self::new(engine.primitives.integer, Data::Integer(integer));
	}

	pub fn new_function(engine: &Engine, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Self {
		return Self::new(engine.primitives.function, Data::Callable(Box::new(Function::new(engine.scope, parameters, block))));
	}

	pub fn new_primitive(engine: &Engine, callback: &'static dyn for<'b> Fn(&'b Engine, Vec<Reference>) -> Reference) -> Self {
		return Self::new(engine.primitives.function, Data::Callable(Box::new(Primitive::new(callback))));
	}

	pub fn new_string(engine: &Engine, string: String) -> Self {
		return Self::new(engine.primitives.string, Data::String(string));
	}

	pub fn get_method(&self, engine: &Engine, name: &String) -> Option<Reference> {
		return self.data.as_class().get_method(engine, name);
	}

	pub fn cast(&self, class: Value) {
		if self.class != class {
			panic!();
		}
	}
}
