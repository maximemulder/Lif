pub mod callable;
pub mod class;
pub mod data;
pub mod instance;

use crate::runtime::{ Engine, Reference };
use data::Data;
use class::Class;
use callable::Callable;
use instance::Instance;

pub struct Value<'a> {
	pub class: *mut Value<'a>,
	data: Data<'a>,
}

impl<'a> Value<'a> {
	pub fn new(class: *mut Value<'a>, data: Data<'a>) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn cast(&self, class: *const Value<'a>) {
		if !std::ptr::eq(self.class, class) {
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

impl<'a> Value<'a> {
	pub fn data_array(&self) -> &Vec<Reference<'a>> {
		data!(self, Array);
	}

	pub fn data_array_mut(&mut self) -> &mut Vec<Reference<'a>> {
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

	pub fn data_class(&self) -> &Class<'a> {
		data!(self, Class);
	}

	pub fn data_class_mut(&mut self) -> &mut Class<'a> {
		data_mut!(self, Class);
	}

	pub fn data_instance(&self) -> &Instance<'a> {
		data!(self, Instance);
	}

	pub fn data_instance_mut(&mut self) -> &mut Instance<'a> {
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
