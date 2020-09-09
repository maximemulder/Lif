use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::GcValue;

pub type GcReference<'a> = GcRef<Reference<'a>>;

pub struct Reference<'a> {
	value: Option<GcValue<'a>>,
	r#type: Type<'a>,
}

enum Type<'a> {
	Variable(GcValue<'a>),
	Constant,
}

impl<'a> Reference<'a> {
	pub fn new_variable(value: Option<GcValue<'a>>, r#type: GcValue<'a>) -> Self {
		return Self {
			value,
			r#type: Type::Variable(r#type),
		};
	}

	pub fn new_constant(value: Option<GcValue<'a>>) -> Self {
		return Self {
			value,
			r#type: Type::Constant,
		};
	}

	pub fn read(&self) -> GcValue<'a> {
		if let Some(value) = self.value {
			return value;
		}

		panic!();
	}

	pub fn write(&mut self, value: GcValue<'a>) {
		match self.r#type {
			Type::Variable(r#type) => if value.isa(r#type) {
				self.set_value(value);
			} else {
				panic!();
			},
			Type::Constant => if self.value.is_none() {
				self.set_value(value);
			} else {
				panic!();
			},
		}
	}

	fn set_value(&mut self, value: GcValue<'a>) {
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
