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
            std::ptr::from_raw_parts_mut::<dyn GcTrace>(self.object, metadata).as_mut().unwrap()
        }
    }

impl Drop for GcGuard {
    fn drop(&mut self) {
        unsafe {
            Box::<dyn GcTrace>::from_raw(transmute::<TraitObject, *mut dyn GcTrace>(self.pointers));
        }
    }
}

impl GcTrace for GcGuard {
    fn trace(&mut self) {
        if !self.flag() {
            self.mark();
            unsafe {
                transmute::<TraitObject, *mut dyn GcTrace>(self.pointers).as_mut().unwrap().trace();
            }
        }
    }
}

pub struct GcRef<T: GcTrace> {
    pointer: Mut<GcGuard>,
    phantom: PhantomData<*const T>,
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

    pub fn get_guard(self) -> Mut<GcGuard> {
        self.pointer
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
            transmute::<*mut (), &T>(self.pointer.object)
        }
    }
}

impl<T: GcTrace> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            transmute::<*mut (), &mut T>(self.pointer.object)
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
