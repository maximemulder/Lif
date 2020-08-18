pub mod engine;
pub mod environment;
pub mod scope;
pub mod value;

pub use engine::Engine;
pub use value::Value;

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Reference<'a> {
	value: *mut Value<'a>,
}

impl<'a> Reference<'a> {
	pub fn new(value: *mut Value<'a>) -> Self {
		return Self {
			value,
		}
	}

	pub fn new_undefined() -> Self {
		return Self {
			value: std::ptr::null_mut(),
		};
	}
}
