use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::GcValue;

pub type GcReference<'a> = GcRef<Reference<'a>>;

pub struct Reference<'a> {
	value: Option<GcValue<'a>>,
	variable: bool,
}

impl<'a> Reference<'a> {
	pub fn new(value: Option<GcValue<'a>>, variable: bool) -> Self {
		return Self {
			value,
			variable,
		};
	}

	pub fn read(&self) -> GcValue<'a> {
		if let Some(value) = self.value {
			return value;
		}

		panic!();
	}

	pub fn write(&mut self, value: GcValue<'a>) {
		if self.variable || self.value.is_none(){
			self.value = Some(value);
		} else {
			panic!();
		}
	}
}

impl GcTraceable for Reference<'_> {
	fn trace(&mut self) {
		if let Some(value) = self.value.as_mut() {
			value.trace();
		}
	}
}
