use crate::memory::{ Mut, Own };

use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::{ Copy, PhantomData };
use std::mem::transmute;
use std::ops::{ Deref, DerefMut };
use std::ptr::{ from_raw_parts_mut, DynMetadata };

pub const GC_THRESHOLD: usize = 250;

pub trait GcTrace {
    fn trace(&mut self);
}

pub struct Gc {
    guards: Vec<Own<GcGuard>>,
    allocations: usize,
}

impl Gc {
    pub fn new() -> Self {
        Self {
            guards: Vec::new(),
            allocations: 0,
        }
    }

    pub fn alloc<T: GcTrace>(&mut self, object: T) -> GcRef<T> {
        let mut guard = Own::new(GcGuard::new(object));
        let r#ref = GcRef::new(guard.get_mut());
        self.guards.push(guard);
        self.allocations += 1;
        r#ref
    }

    pub fn collect(&mut self) {
        // self.guards.drain_filter(|guard| guard.reset());
        self.allocations = 0;
    }

    pub fn get_allocations(&self) -> usize {
        self.allocations
    }
}

pub struct GcGuard {
    flag: bool,
    metadata: *const (),
    object: *mut (),
}

impl GcGuard {
    fn new<T: GcTrace>(mut object: T) -> Self {
        unsafe {
            let pointer: *mut dyn GcTrace = &mut object;
            let (object, metadata) = pointer.to_raw_parts();
            Self {
                flag: false,
                metadata: transmute::<DynMetadata<dyn GcTrace>, *const ()>(metadata),
                object,
            }
        }
    }

    fn object(&mut self) -> &mut dyn GcTrace {
        unsafe {
            let metadata = transmute::<*const (), DynMetadata<dyn GcTrace>>(self.metadata);
            from_raw_parts_mut::<dyn GcTrace>(self.object, metadata).as_mut().unwrap()
        }
    }
}

impl Drop for GcGuard {
    fn drop(&mut self) {
        unsafe {
            Box::<dyn GcTrace>::from_raw(self.object());
        }
    }
}

impl GcTrace for GcGuard {
    fn trace(&mut self) {
        if !self.flag {
            self.flag = true;
            self.object().trace();
        }
    }
}

pub struct GcRef<T: GcTrace> {
    guard: Mut<GcGuard>,
    phantom: PhantomData<*const T>,
}

impl<T: GcTrace> GcRef<T> {
    pub fn null() -> Self {
        Self {
            guard: Mut::null(),
            phantom: PhantomData,
        }
    }

    fn new(guard: Mut<GcGuard>) -> Self {
        Self {
            guard,
            phantom: PhantomData,
        }
    }

    pub fn get_guard(self) -> Mut<GcGuard> {
        self.guard
    }
}

impl<T: GcTrace> GcTrace for GcRef<T> {
    fn trace(&mut self) {
        self.guard.trace();
    }
}

impl<T: GcTrace> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            transmute::<*mut (), &T>(self.guard.object)
        }
    }
}

impl<T: GcTrace> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            transmute::<*mut (), &mut T>(self.guard.object)
        }
    }
}

impl<T: GcTrace> PartialEq for GcRef<T> {
    fn eq(&self, other: &GcRef<T>) -> bool {
        self.guard == other.guard
    }
}

impl<T: GcTrace> Eq for GcRef<T> {}

impl<T: GcTrace> Clone for GcRef<T> {
    fn clone(&self) -> Self {
        Self {
            guard: self.guard,
            phantom: PhantomData,
        }
    }
}

impl<T: GcTrace> Copy for GcRef<T> {}
