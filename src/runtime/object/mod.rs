pub mod callable;
pub mod class;
pub mod data;
pub mod instance;

use crate::runtime::{ Engine, Reference, Value };
use data::Data;
use class::Class;
use callable::Callable;
use instance::Instance;

pub struct Object<'a> {
	pub class: Value,
	data: Data<'a>,
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
		return engine.get_object(self.class).data_class().get_method(engine, name);
	}
}

macro_rules! data {
	( $this:expr, $variant:ident ) => {
		if let Data::$variant(variant) = &$this.data {
			return variant;
		}

		panic!();
	};
}

macro_rules! data_mut {
	( $this:expr, $variant:ident ) => {
		if let Data::$variant(variant) = &mut $this.data {
			return variant;
		}

		panic!();
	};
}

impl<'a> Object<'a> {
	pub fn data_array(&self) -> &Vec<Reference> {
		data!(self, Array);
	}

	pub fn data_array_mut(&mut self) -> &mut Vec<Reference> {
		data_mut!(self, Array);
	}

	pub fn data_boolean(&self) -> &bool {
		data!(self, Boolean);
	}

	pub fn data_boolean_mut(&mut self) -> &mut bool {
		data_mut!(self, Boolean);
	}

	pub fn data_callable(&self) -> &Box<dyn Callable<'a> + 'a> {
		data!(self, Callable);
	}

	pub fn data_callable_mut(&mut self) -> &mut Box<dyn Callable<'a> + 'a> {
		data_mut!(self, Callable);
	}

	pub fn data_class(&self) -> &Class {
		data!(self, Class);
	}

	pub fn data_class_mut(&mut self) -> &mut Class {
		data_mut!(self, Class);
	}

	pub fn data_instance(&self) -> &Instance {
		data!(self, Instance);
	}

	pub fn data_instance_mut(&mut self) -> &mut Instance {
		data_mut!(self, Instance);
	}

	pub fn data_integer(&self) -> &usize {
		data!(self, Integer);
	}

	pub fn data_integer_mut(&mut self) -> &mut usize {
		data_mut!(self, Integer);
	}

	pub fn data_string(&self) -> &String {
		data!(self, String);
	}

	pub fn data_string_mut(&mut self) -> &mut String {
		data_mut!(self, String);
	}
}
