mod object;

pub use object::ValueObject;

use crate::runtime::data::Data;

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Value<'a> {
	object: *mut ValueObject<'a>,
}

impl<'a> Value<'a> {
	pub fn create(class: Value<'a>, data: Data<'a>) -> Self {
		return Self::new(Box::into_raw(Box::new(ValueObject::new(class, data))));
	}

	pub fn new(object: *mut ValueObject<'a>) -> Self {
		return Self {
			object,
		}
	}

	pub fn new_undefined() -> Self {
		return Self {
			object: std::ptr::null_mut(),
		};
	}

	pub fn object_ref(&self) -> &ValueObject<'a> {
		return unsafe { self.object.as_ref().unwrap() };
	}

	pub fn object_mut(&mut self) -> &mut ValueObject<'a> {
		return unsafe { self.object.as_mut().unwrap() };
	}
}
