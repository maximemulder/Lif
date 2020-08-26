mod object;

pub use object::ScopeObject;

use crate::runtime::proxy::Proxy;

pub type Scope<'a> = Proxy<ScopeObject<'a>>;

impl<'a> Scope<'a> {
	pub fn new() -> Self {
		return Self::alloc(ScopeObject::new());
	}

	pub fn new_child(parent: Scope<'a>) -> Self {
		return Self::alloc(ScopeObject::new_child(parent));
	}
}
