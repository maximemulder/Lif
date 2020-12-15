use crate::memory::{ Mut, Ref };

use std::ops::{ Deref, DerefMut, Drop };

fn alloc<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

fn dealloc<T: ?Sized>(pointer: *mut T) {
    unsafe {
        Box::from_raw(pointer);
    }
}

pub struct Own<T: ?Sized> {
    pointer: *mut T,
}

impl<T> Own<T> {
    pub fn new(value: T) -> Self {
        Self {
            pointer: alloc(value),
        }
    }
}

impl<T: ?Sized> Own<T> {
    pub fn get_ref(&self) -> Ref<T> {
        Ref::new(self.pointer)
    }

    pub fn get_mut(&mut self) -> Mut<T> {
        Mut::new(self.pointer)
    }
}

impl<T: ?Sized> Drop for Own<T> {
    fn drop(&mut self) {
        dealloc(self.pointer);
    }
}

impl<T: ?Sized> Deref for Own<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for Own<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.pointer.as_mut().unwrap() }
    }
}
