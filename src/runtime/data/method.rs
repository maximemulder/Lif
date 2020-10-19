use crate::runtime::gc::GcTraceable;
use crate::runtime::value::GcValue;

pub struct Method<'a, 'b> {
	pub function: GcValue<'a, 'b>,
	pub this: GcValue<'a, 'b>,
}

impl<'a, 'b> Method<'a, 'b> {
	pub fn new(function: GcValue<'a, 'b>, this: GcValue<'a, 'b>) -> Self {
		Self {
			function,
			this,
		}
	}
}

impl GcTraceable for Method<'_, '_> {
    fn trace(&mut self) {
		self.function.trace();
		self.this.trace();
    }
}
