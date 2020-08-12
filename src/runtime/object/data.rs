use crate::runtime::{ Reference };
use super::class::Class;
use super::callable::Callable;
use super::instance::Instance;

pub enum Data<'a> {
	Array(Vec<Reference>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a> + 'a>),
	Class(Class),
	Instance(Instance),
	Integer(usize),
	String(String),
	Null,
}

macro_rules! as_variant {
	( $this:expr, $variant:ident ) => {
		if let Data::$variant(variant) = $this {
			return variant;
		}

		panic!();
	};
}

impl<'a> Data<'a> {
	pub fn as_array(&self) -> &Vec<Reference> {
		as_variant!(self, Array);
	}

	pub fn as_boolean(&self) -> &bool {
		as_variant!(self, Boolean);
	}

	pub fn as_callable(&self) -> &Box<dyn Callable<'a> + 'a> {
		as_variant!(self, Callable);
	}

	pub fn as_class(&self) -> &Class {
		as_variant!(self, Class);
	}

	pub fn as_instance(&self) -> &Instance {
		as_variant!(self, Instance);
	}

	pub fn as_integer(&self) -> &usize {
		as_variant!(self, Integer);
	}

	pub fn as_string(&self) -> &String {
		as_variant!(self, String);
	}

	pub fn as_array_mut(&mut self) -> &mut Vec<Reference> {
		as_variant!(self, Array);
	}

	pub fn as_boolean_mut(&mut self) -> &mut bool {
		as_variant!(self, Boolean);
	}

	pub fn as_callable_mut(&mut self) -> &mut Box<dyn Callable<'a> + 'a> {
		as_variant!(self, Callable);
	}

	pub fn as_class_mut(&mut self) -> &mut Class {
		as_variant!(self, Class);
	}

	pub fn as_instance_mut(&mut self) -> &mut Instance {
		as_variant!(self, Instance);
	}

	pub fn as_integer_mut(&mut self) -> &mut usize {
		as_variant!(self, Integer);
	}

	pub fn as_string_mut(&mut self) -> &mut String {
		as_variant!(self, String);
	}
}
