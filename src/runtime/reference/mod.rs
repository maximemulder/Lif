mod object;

pub use object::ReferenceObject;

use crate::runtime::value::Value;
use crate::runtime::proxy::Proxy;

pub type Reference<'a> = Proxy<ReferenceObject<'a>>;

impl<'a> Reference<'a> {
	pub fn new(value: Value<'a>) -> Self {
		return Self::alloc(ReferenceObject::new(value));
	}

	pub fn new_undefined() -> Self {
		return Self::new(Value::new_undefined());
	}
}
