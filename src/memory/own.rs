use crate::memory::{ Mut, Ref };

use std::ops::{ Deref, DerefMut, Drop };

fn alloc<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

fn dealloc<T>(pointer: *mut T) {
    unsafe {
        Box::from_raw(pointer);
    }
}

pub struct Own<T> {
    pointer: *mut T,
}

impl<T> Own<T> {
    pub fn new(value: T) -> Self {
        Self {
            pointer: alloc(value),
        }
    }

    pub fn get_ref(&self) -> Ref<T> {
        Ref::new(self.pointer)
    }

    pub fn get_mut(&mut self) -> Mut<T> {
        Mut::new(self.pointer)
    }
}

impl<T> Drop for Own<T> {
    fn drop(&mut self) {
        dealloc(self.pointer);
    }
}

impl<T> Deref for Own<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T> DerefMut for Own<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.pointer.as_mut().unwrap() }
    }
}
