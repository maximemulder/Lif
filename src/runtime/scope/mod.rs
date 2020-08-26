mod object;

pub use object::ScopeObject;

use crate::runtime::proxy::Proxy;
use std::ops::DerefMut;

pub type Scope<'a> = Proxy<ScopeObject<'a>>;

impl<'a> Scope<'a> {
	pub fn new() -> Self {
		return Self::alloc(ScopeObject::new());
	}

	pub fn new_child(parent: Scope<'a>) -> Self {
		return Self::alloc(ScopeObject::new_child(parent));
	}

	pub fn visit(&mut self) {
		if !Proxy::get_flag(self) {
			Proxy::mark(self);
			self.deref_mut().visit();
		}
	}
}
