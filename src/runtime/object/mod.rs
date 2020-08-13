pub mod callable;
pub mod class;
pub mod data;
pub mod instance;

use crate::runtime::{ Engine, Reference, Value };
use data::Data;

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

	pub fn cast(&self, class: Value) {
		if self.class != class {
			panic!();
		}
	}

	pub fn get_method(&self, engine: &Engine, name: &str) -> Option<Reference> {
		return engine.get_object(self.class).data.as_class().get_method(engine, name);
	}
}
