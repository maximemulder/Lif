use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::GcValue;

pub type GcReference<'a> = GcRef<Reference<'a>>;

pub struct Reference<'a> {
	value: Option<GcValue<'a>>,
}

impl<'a> Reference<'a> {
	pub fn new(value: GcValue<'a>) -> Self {
		return Self {
			value: Some(value),
		};
	}

	pub fn new_undefined() -> Self {
		return Self {
			value: None,
		};
	}

	pub fn read(&self) -> GcValue<'a> {
		if let Some(value) = self.value {
			return value;
		}

		panic!();
	}

	pub fn write(&mut self, value: GcValue<'a>) {
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
