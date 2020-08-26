use std::ops::{ Deref, DerefMut };
use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::Copy;

pub trait Visitable {
	fn visit(&mut self);
}

struct Object<T> {
	object: T,
	flag: bool,
}

impl<T> Object<T> {
	pub fn new(object: T) -> Self {
		return Self {
			object,
			flag: false,
		};
	}
}

pub struct Proxy<T> {
	pointer: *mut Object<T>,
}

impl<T> Proxy<T> {
	pub fn alloc(object: T) -> Self {
		return Self {
			pointer: Box::into_raw(Box::new(Object::new(object))),
		};
	}

	pub fn null() -> Self {
		return Self {
			pointer: std::ptr::null_mut(),
		};
	}

	pub fn mark(proxy: &mut Proxy<T>) {
		unsafe { proxy.pointer.as_mut().unwrap() }.flag = true;
	}

	pub fn collect(proxy: &mut Proxy<T>) -> bool {
		if Self::get_flag(proxy) {
			unsafe { proxy.pointer.as_mut().unwrap() }.flag = false;
			return true;
		} else {
			unsafe { Box::from_raw(proxy.pointer); };
			return false;
		}
	}

	pub fn get_flag(proxy: &Proxy<T>) -> bool {
		unsafe {
			return if let Some(thing) = proxy.pointer.as_ref() {
				thing.flag
			} else {
				true
			}
		}
	}
}

impl<T: Visitable> Visitable for Proxy<T> {
	fn visit(&mut self) {
		if !Proxy::get_flag(self) {
			Proxy::mark(self);
			self.deref_mut().visit();
		}
	}
}

impl<T> Deref for Proxy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
		return &unsafe { self.pointer.as_ref().unwrap() }.object;
    }
}

impl<T> DerefMut for Proxy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut unsafe { self.pointer.as_mut().unwrap() }.object;
    }
}

impl<T> PartialEq for Proxy<T> {
	fn eq(&self, other: &Proxy<T>) -> bool {
		return self.pointer == other.pointer;
	}
}

impl<T> Eq for Proxy<T> {}

impl<T> Clone for Proxy<T> {
    fn clone(&self) -> Self {
		return Self {
			pointer: self.pointer,
		};
    }
}

impl<T> Copy for Proxy<T> {}
