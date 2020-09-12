use std::ops::{ Deref, DerefMut };
use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::Copy;

pub trait GcTraceable {
	fn trace(&mut self);
}

pub struct Gc<T> {
	refs: Vec<GcRef<T>>,
}

impl<T> Gc<T> {
	pub fn new() -> Self {
		return Self {
			refs: Vec::new(),
		};
	}

	pub fn alloc(&mut self, object: T) -> GcRef<T> {
		let r#ref = GcRef::alloc(object);
		self.refs.push(r#ref);
		return r#ref;
	}

	pub fn collect(&mut self) -> usize {
		let mut i = 0;
		self.refs.drain_filter(|r#ref| if !r#ref.collect() {
			i += 1;
			true
		} else {
			false
		});
		return i;
	}
}

struct GcObject<T> {
	object: T,
	flag: bool,
}

impl<T> GcObject<T> {
	fn new(object: T) -> Self {
		return Self {
			object,
			flag: false,
		};
	}
}

pub struct GcRef<T> {
	pointer: *mut GcObject<T>,
}

impl<T> GcRef<T> {
	pub fn null() -> Self {
		return Self {
			pointer: std::ptr::null_mut(),
		};
	}

	pub fn alloc(object: T) -> Self {
		return Self {
			pointer: Box::into_raw(Box::new(GcObject::new(object))),
		};
	}

	fn mark(&mut self) {
		unsafe { self.pointer.as_mut().unwrap() }.flag = true;
	}

	fn collect(&mut self) -> bool {
		if self.flag() {
			unsafe { self.pointer.as_mut().unwrap() }.flag = false;
			return true;
		} else {
			unsafe { Box::from_raw(self.pointer); };
			return false;
		}
	}

	fn flag(&self) -> bool {
		return if let Some(thing) = unsafe { self.pointer.as_ref() } {
			thing.flag
		} else {
			true
		};
	}
}

impl<T: GcTraceable> GcTraceable for GcRef<T> {
	fn trace(&mut self) {
		if !self.flag() {
			self.mark();
			self.deref_mut().trace();
		}
	}
}

impl<T> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
		return &unsafe { self.pointer.as_ref().unwrap() }.object;
    }
}

impl<T> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut unsafe { self.pointer.as_mut().unwrap() }.object;
    }
}

impl<T> PartialEq for GcRef<T> {
	fn eq(&self, other: &GcRef<T>) -> bool {
		return self.pointer == other.pointer;
	}
}

impl<T> Eq for GcRef<T> {}

impl<T> Clone for GcRef<T> {
    fn clone(&self) -> Self {
		return Self {
			pointer: self.pointer,
		};
    }
}

impl<T> Copy for GcRef<T> {}
