use crate::memory::{ Mut, Own };

use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::{ Copy, PhantomData };
use std::mem::transmute;
use std::ops::{ Deref, DerefMut };
use std::raw::TraitObject;

pub const GC_THRESHOLD: usize = 250;

pub trait GcTrace {
    fn trace(&mut self);
}

pub struct Gc {
    guards: Vec<Own<GcGuard>>,
}

impl Gc {
    pub fn new() -> Self {
        Self {
            guards: Vec::new(),
        }
    }

    pub fn alloc<T: GcTrace>(&mut self, value: T) -> GcRef<T> {
        unsafe {
            let pointers: *mut dyn GcTrace = Box::into_raw(Box::new(value));
            let mut guard = Own::new(GcGuard::new(transmute::<*mut dyn GcTrace, TraitObject>(pointers)));
            let r#ref = GcRef::new(guard.get_mut());
            self.guards.push(guard);
            r#ref
        }
    }

    pub fn collect(&mut self) {
        self.guards.drain_filter(|guard| guard.reset());
    }
}

struct GcGuard {
    pointers: TraitObject,
    flag: bool,
}

impl GcGuard {
    fn new(pointers: TraitObject) -> Self {
        Self {
            pointers,
            flag: false,
        }
    }

    fn mark(&mut self) {
        self.flag = true;
    }

    fn flag(&mut self) -> bool {
        self.flag
    }

    fn reset(&mut self) -> bool {
        let reset = !self.flag;
        self.flag = false;
        reset
    }
}

impl Drop for GcGuard {
    fn drop(&mut self) {
        unsafe {
            Box::<dyn GcTrace>::from_raw(transmute::<TraitObject, *mut dyn GcTrace>(self.pointers));
        };
    }
}

pub struct GcRef<T: GcTrace> {
    pointer: Mut<GcGuard>,
    phantom: PhantomData<T>,
}

impl<T: GcTrace> GcRef<T> {
    pub fn null() -> Self {
        Self {
            pointer: Mut::null(),
            phantom: PhantomData,
        }
    }

    fn new(pointer: Mut<GcGuard>) -> Self {
        Self {
            pointer,
            phantom: PhantomData,
        }
    }
}

impl<T: GcTrace> GcTrace for GcRef<T> {
    fn trace(&mut self) {
        unsafe {
            if !self.pointer.flag() {
                self.pointer.mark();
                transmute::<*mut (), *mut T>(self.pointer.pointers.data).as_mut().unwrap().trace()
            }
        }
    }
}

impl<T: GcTrace> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            transmute::<*mut (), *mut T>(self.pointer.pointers.data).as_ref().unwrap()
        }
    }
}

impl<T: GcTrace> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            transmute::<*mut (), *mut T>(self.pointer.pointers.data).as_mut().unwrap()
        }
    }
}

impl<T: GcTrace> PartialEq for GcRef<T> {
    fn eq(&self, other: &GcRef<T>) -> bool {
        self.pointer == other.pointer
    }
}

impl<T: GcTrace> Eq for GcRef<T> {}

impl<T: GcTrace> Clone for GcRef<T> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer,
            phantom: PhantomData,
        }
    }
}

impl<T: GcTrace> Copy for GcRef<T> {}
