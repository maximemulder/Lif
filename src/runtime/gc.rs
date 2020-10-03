use std::ops::{ Deref, DerefMut };
use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::Copy;

pub const GC_THRESHOLD: usize = 250;

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
		let r#ref = GcRef::new(Box::into_raw(Box::new(GcObject::new(object))));
		self.refs.push(r#ref);
		return r#ref;
	}

	pub fn collect(&mut self) {
		self.refs.drain_filter(|r#ref| !r#ref.collect());
	}
}

impl<T> Drop for Gc<T> {
    fn drop(&mut self) {
        for r#ref in self.refs.iter_mut() {
			r#ref.free();
		}
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

	fn new(pointer: *mut GcObject<T>) -> Self {
		return Self {
			pointer,
		};
	}

	fn mark(&mut self) {
		unsafe { self.pointer.as_mut() }.unwrap().flag = true;
	}

	fn flag(&self) -> bool {
		return unsafe { self.pointer.as_ref() }.unwrap().flag;
	}

	fn reset(&mut self) {
		unsafe { self.pointer.as_mut() }.unwrap().flag = false;
	}

	fn free(&mut self) {
		unsafe { Box::from_raw(self.pointer); };
	}

	fn collect(&mut self) -> bool {
		let flag = self.flag();
		if flag {
			self.reset();
		} else {
			self.free();
		}

		return flag;
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
