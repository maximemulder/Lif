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

impl Data<'_> {
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

	pub fn as_class(&mut self) -> &mut Class {
		if let Data::Class(class) = self {
			return class;
		}

		panic!();
	}
}