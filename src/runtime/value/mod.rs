mod object;

pub use object::ValueObject;

use crate::runtime::data::Data;
use crate::runtime::proxy::Proxy;

pub type Value<'a> = Proxy<ValueObject<'a>>;

impl<'a> Value<'a> {
	pub fn new(class: Value<'a>, data: Data<'a>) -> Self {
		return Self::alloc(ValueObject::new(class, data));
	}

	pub fn new_undefined() -> Self {
		return Self::null();
	}
}
