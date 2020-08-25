use std::ops::{ Deref, DerefMut };
use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::Copy;

pub struct Proxy<T> {
	pointer: *mut T,
	flag: bool,
}

impl<T> Proxy<T> {
	pub fn alloc(value: T) -> Self {
		return Self {
			pointer: Box::into_raw(Box::new(value)),
			flag: true,
		};
	}

	pub fn null() -> Self {
		return Self {
			pointer: std::ptr::null_mut(),
			flag: true,
		};
	}
}

impl<T> Deref for Proxy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
		return unsafe { self.pointer.as_ref().unwrap() };
    }
}

impl<T> DerefMut for Proxy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		return unsafe { self.pointer.as_mut().unwrap() };
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
			flag: true,
		};
    }
}

impl<T> Copy for Proxy<T> {}
