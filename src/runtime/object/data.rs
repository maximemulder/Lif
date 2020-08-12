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

impl<'a> Data<'a> {
	pub fn as_array(&mut self) -> &mut Vec<Reference> {
		if let Data::Array(array) = self {
			return array;
		}

		panic!();
	}

	pub fn as_boolean(&mut self) -> &mut bool {
		if let Data::Boolean(boolean) = self {
			return boolean;
		}

		panic!();
	}

	pub fn as_callable(&mut self) -> &mut Box<dyn Callable<'a> + 'a> {
		if let Data::Callable(callable) = self {
			return callable;
		}

		panic!();
	}

	pub fn as_class(&mut self) -> &mut Class {
		if let Data::Class(class) = self {
			return class;
		}

		panic!();
	}

	pub fn as_string(&mut self) -> &mut String {
		if let Data::String(string) = self {
			return string;
		}

		panic!();
	}
}
