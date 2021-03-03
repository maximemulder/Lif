use std::ops::Deref;
use std::ptr::null;

pub struct Ref<T: ?Sized> {
    pointer: *const T,
}

impl<T> Ref<T> {
    pub fn null() -> Self {
        Self {
            pointer: null(),
        }
    }
}

impl<T: ?Sized> Ref<T> {
    pub fn new(pointer: *const T) -> Self {
        Self {
            pointer,
        }
    }

    pub fn from_ref(reference: &T) -> Self {
        Self {
            pointer: reference as *const T
        }
    }

    pub fn as_ref(r#ref: &Self) -> &T {
        unsafe { r#ref.pointer.as_ref().unwrap() }
    }

    pub fn as_option(r#ref: &Option<Ref<T>>) -> Option<&T> {
        r#ref.as_ref().map(Ref::as_ref)
    }
}

impl<T: ?Sized> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T: ?Sized> PartialEq for Ref<T> {
    fn eq(&self, other: &Ref<T>) -> bool {
        self.pointer == other.pointer
    }
}

impl<T: ?Sized> Eq for Ref<T> {}

impl<T: ?Sized> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self::new(self.pointer)
    }
}

impl<T: ?Sized> Copy for Ref<T> {}
