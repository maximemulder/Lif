use crate::runtime::Object;

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Value<'a> {
	object: *mut Object<'a>,
}

impl<'a> Value<'a> {
	pub fn create(object: Object<'a>) -> Self {
		return Self {
			object: Box::into_raw(Box::new(object)),
		}
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
