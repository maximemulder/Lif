use std::ops::{ Deref, DerefMut };
use std::ptr::null_mut;

pub struct Mut<T: ?Sized> {
    pointer: *mut T,
}

impl<T> Mut<T> {
    pub fn null() -> Self {
        Self {
            pointer: null_mut(),
        }
    }
}

impl<T: ?Sized> Mut<T> {
    pub fn new(pointer: *mut T) -> Self {
        Self {
            pointer,
        }
    }
}

impl<T: ?Sized> Deref for Mut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for Mut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.pointer.as_mut().unwrap() }
    }
}

impl<T: ?Sized> PartialEq for Mut<T> {
    fn eq(&self, other: &Mut<T>) -> bool {
        self.pointer == other.pointer
    }
}

impl<T: ?Sized> Eq for Mut<T> {}

impl<T: ?Sized> Clone for Mut<T> {
    fn clone(&self) -> Self {
        Self::new(self.pointer)
    }
}

impl<T: ?Sized> Copy for Mut<T> {}
