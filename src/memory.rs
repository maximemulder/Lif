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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ref<T> {
    pointer: *const T,
}

impl<T> Ref<T> {
    fn new(pointer: *mut T) -> Self {
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mut<T> {
    pointer: *mut T,
}

impl<T> Mut<T> {
    fn new(pointer: *mut T) -> Self {
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
