use super::class::Class;
use super::callable::Callable;
use super::instance::Instance;

pub enum Data {
	Array(Vec<usize>),
	Boolean(bool),
	Class(Class),
	Instance(Instance),
	Integer(usize),
	Callable(Box<dyn Callable>),
	String(String),
	Undefined(()),
}

impl Data {
	pub fn as_array(&mut self) -> &mut Vec<usize> {
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
