use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::Value;

pub struct Reference<'a> {
	value: Option<GcRef<Value<'a>>>,
}

impl<'a> Reference<'a> {
	pub fn new(value: GcRef<Value<'a>>) -> Self {
		return Self {
			value: Some(value),
		};
	}

	pub fn new_undefined() -> Self {
		return Self {
			value: None,
		};
	}

	pub fn read(&self) -> GcRef<Value<'a>> {
		if let Some(value) = self.value {
			return value;
		}

		panic!();
	}

	pub fn write(&mut self, value: GcRef<Value<'a>>) {
		self.value = Some(value);
	}
}

impl GcTraceable for Reference<'_> {
	fn trace(&mut self) {
		if let Some(value) = self.value.as_mut() {
			value.trace();
		}
	}
}
