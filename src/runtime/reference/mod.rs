mod object;

pub use object::ReferenceObject;

use crate::runtime::value::Value;
use std::ops::{ Deref, DerefMut };

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Reference<'a> {
	object: *mut ReferenceObject<'a>,
}

impl<'a> Reference<'a> {
	pub fn new(value: Value<'a>) -> Self {
		return Self {
			object: Box::into_raw(Box::new(ReferenceObject::new(value))),
		}
	}

	pub fn new_undefined() -> Self {
		return Self::new(Value::new_undefined());
	}
}

impl<'a> Deref for Reference<'a> {
    type Target = ReferenceObject<'a>;

    fn deref(&self) -> &Self::Target {
		return unsafe { self.object.as_ref().unwrap() };
    }
}

impl<'a> DerefMut for Reference<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		return unsafe { self.object.as_mut().unwrap() };
    }
}
