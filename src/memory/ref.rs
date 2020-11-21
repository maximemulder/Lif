use std::ptr::null;
use std::ops::Deref;

pub struct Ref<T> {
    pointer: *const T,
}

impl<T> Ref<T> {
    pub fn null() -> Self {
        Self {
            pointer: null(),
        }
    }

    pub fn new(pointer: *mut T) -> Self {
        Self {
            pointer,
        }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T> PartialEq for Ref<T> {
    fn eq(&self, other: &Ref<T>) -> bool {
        self.pointer == other.pointer
    }
}

impl<T> Eq for Ref<T> {}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer,
        }
    }
}

impl<T> Copy for Ref<T> {}
