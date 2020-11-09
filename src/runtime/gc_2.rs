use std::ops::{ Deref, DerefMut };
use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::{ Copy, PhantomData };

pub const GC_THRESHOLD: usize = 250;

pub trait GcTraceable {
    fn trace(&mut self);
}

pub struct Gc<T> {
    objects: Vec<*mut GcObject<T>>,
}

impl<T> Gc<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn alloc<'a>(&'a mut self, value: T) -> GcRef<'a, T> {
        let object = Box::into_raw(Box::new(GcObject::new(value)));
        self.objects.push(object);
        GcRef::new(object)
    }

    pub fn collect(&mut self) {
        self.objects.drain_filter(|pointer| {
            let object = unsafe { pointer.as_mut() }.unwrap();
            let flag = object.flag();
            if flag {
                object.reset();
            } else {
                unsafe { Box::from_raw(pointer); };
            }

            flag
        });
    }
}

impl<T> Drop for Gc<T> {
    fn drop(&mut self) {
        for pointer in self.objects.iter_mut() {
            unsafe { Box::from_raw(pointer); };
        }
    }
}

struct GcObject<T> {
    object: T,
    flag: bool,
}

impl<T> GcObject<T> {
    fn new(object: T) -> Self {
        Self {
            object,
            flag: false,
        }
    }

    fn flag(&self) -> bool {
        self.flag
    }

    fn mark(&mut self) {
        self.flag = true;
    }

    fn reset(&mut self) {
        self.flag = false;
    }
}

pub struct GcRef<'a, T> {
    pointer: *mut GcObject<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> GcRef<'a, T> {
    pub fn null() -> Self {
        Self {
            pointer: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }

    fn new(pointer: *mut GcObject<T>) -> Self {
        Self {
            pointer,
            phantom: PhantomData,
        }
    }

    fn object(&mut self) -> &mut GcObject<T> {
        unsafe { self.pointer.as_mut() }.unwrap()
    }
}

impl<T: GcTraceable> GcTraceable for GcRef<'_, T> {
    fn trace(&mut self) {
        let object = self.object();
        if !object.flag() {
            object.mark();
            self.deref_mut().trace();
        }
    }
}

impl<T> Deref for GcRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.pointer.as_ref().unwrap() }.object
    }
}

impl<T> DerefMut for GcRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut unsafe { self.pointer.as_mut().unwrap() }.object
    }
}

impl<T> PartialEq for GcRef<'_, T> {
    fn eq(&self, other: &GcRef<T>) -> bool {
        self.pointer == other.pointer
    }
}

impl<T> Eq for GcRef<'_, T> {}

impl<T> Clone for GcRef<'_, T> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for GcRef<'_, T> {}
