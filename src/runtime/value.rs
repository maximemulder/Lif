use crate::runtime::Object;
use crate::runtime::object::data::Data;

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Value<'a> {
	object: *mut Object<'a>,
}

impl<'a> Value<'a> {
	pub fn create(class: Value<'a>, data: Data<'a>) -> Self {
		return Self::new(Box::into_raw(Box::new(Object::new(class, data))));
	}

	pub fn new(object: *mut Object<'a>) -> Self {
		return Self {
			object,
		}
	}

	pub fn new_undefined() -> Self {
		return Self {
			object: std::ptr::null_mut(),
		};
	}

	pub fn object_ref(&self) -> &Object<'a> {
		return unsafe { self.object.as_ref().unwrap() };
	}

	pub fn object_mut(&mut self) -> &mut Object<'a> {
		return unsafe { self.object.as_mut().unwrap() };
	}
}
