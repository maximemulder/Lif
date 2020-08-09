mod function;
mod instance;
mod class;

pub use function::Function;
pub use instance::Instance;
pub use class::Class;

pub enum Data {
	Array(Vec<usize>),
	Boolean(bool),
	Class(Class),
	Instance(Instance),
	Integer(usize),
	Function(Function),
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
