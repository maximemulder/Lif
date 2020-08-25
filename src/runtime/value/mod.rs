mod object;

pub use object::ValueObject;

use crate::runtime::data::Data;
use std::ops::{ Deref, DerefMut };

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
}

impl<'a> Deref for Value<'a> {
    type Target = ValueObject<'a>;

    fn deref(&self) -> &Self::Target {
		return unsafe { self.object.as_ref().unwrap() };
    }
}

impl<'a> DerefMut for Value<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		return unsafe { self.object.as_mut().unwrap() };
    }
}
