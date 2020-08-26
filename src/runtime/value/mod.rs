mod object;

pub use object::ValueObject;

use crate::runtime::data::Data;
use crate::runtime::proxy::Proxy;
use std::ops::DerefMut;

pub type Value<'a> = Proxy<ValueObject<'a>>;

impl<'a> Value<'a> {
	pub fn new(class: Value<'a>, data: Data<'a>) -> Self {
		return Self::alloc(ValueObject::new(class, data));
	}

	pub fn new_undefined() -> Self {
		return Self::null();
	}

	pub fn visit(&mut self) {
		if !Proxy::get_flag(self) {
			Proxy::mark(self);
			self.deref_mut().visit();
		}
	}
}
