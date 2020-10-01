use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;

pub struct Generic<'a, 'b> {
	pub reference: GcReference<'a, 'b>,
	pub scope: GcScope<'a, 'b>,
	pub generics: &'b Vec<&'a str>,
}

impl<'a, 'b> Generic<'a, 'b> {
	pub fn new(reference: GcReference<'a, 'b>, scope: GcScope<'a, 'b>, generics: &'b Vec<&'a str>) -> Self {
		return Self {
			reference,
			scope,
			generics,
		};
	}
}

impl GcTraceable for Generic<'_, '_> {
	fn trace(&mut self) {
		self.reference.trace();
		self.scope.trace();
	}
}
