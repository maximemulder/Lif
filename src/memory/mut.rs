use std::ops::{ Deref, DerefMut };
use std::ptr::null_mut;

pub struct Mut<T> {
    pointer: *mut T,
}

impl<T> Mut<T> {
    pub fn null() -> Self {
        Self {
            pointer: null_mut(),
        }
    }

    pub fn new(pointer: *mut T) -> Self {
        Self {
            pointer,
        }
    }
}

impl<T> Deref for Mut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T> DerefMut for Mut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.pointer.as_mut().unwrap() }
    }
}

impl<T> PartialEq for Mut<T> {
    fn eq(&self, other: &Mut<T>) -> bool {
        self.pointer == other.pointer
    }
}

impl<T> Eq for Mut<T> {}

impl<T> Clone for Mut<T> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer,
        }
    }
}

impl<T> Copy for Mut<T> {}
