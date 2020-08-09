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
	pub fn as_array(&self) -> Vec<usize> {
		if let Data::Array(array) = self {
			return array.clone();
		}

		panic!();
	}

	pub fn as_boolean(&self) -> bool {
		if let Data::Boolean(boolean) = self {
			return *boolean;
		}

		panic!();
	}
}
