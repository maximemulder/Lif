use std::clone::Clone;
use std::cmp::{ Eq, PartialEq };
use std::marker::{ Copy, PhantomData };
use std::mem::transmute;
use std::ops::{ Deref, DerefMut };
use std::ptr::NonNull;
use std::raw::TraitObject;

pub const GC_THRESHOLD: usize = 250;

pub trait GcTrace {
    fn trace(&mut self);
}

pub struct Gc {
    guards: Vec<NonNull<GcGuard>>,
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
            let guard = NonNull::new_unchecked(Box::into_raw(Box::new(GcGuard::new(transmute::<*mut dyn GcTrace, TraitObject>(pointers)))));
            self.guards.push(guard);
            GcRef::new(guard)
        }
    }

    pub fn collect(&mut self) {
        self.guards.drain_filter(|guard| !{
            let guard = unsafe { guard.as_mut() };
            let flag = guard.flag();
            if flag {
                guard.reset();
            } else {
                unsafe { Box::from_raw(guard); };
            }

            flag
        });
    }
}

impl Drop for Gc {
    fn drop(&mut self) {
        unsafe {
            for guard in self.guards.iter_mut() {
                Box::from_raw(guard.as_mut());
            }
        }
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

    fn flag(&self) -> bool {
        self.flag
    }

    fn reset(&mut self) {
        self.flag = false;
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
    pointer: NonNull<GcGuard>,
    phantom: PhantomData<T>,
}

impl<T: GcTrace> GcRef<T> {
    pub fn null() -> Self {
        Self {
            pointer: NonNull::dangling(),
            phantom: PhantomData,
        }
    }

    fn new(pointer: NonNull<GcGuard>) -> Self {
        Self {
            pointer,
            phantom: PhantomData,
        }
    }
}

impl<T: GcTrace> GcTrace for GcRef<T> {
    fn trace(&mut self) {
        unsafe {
            let guard = self.pointer.as_mut();
            if !guard.flag() {
                guard.mark();
                transmute::<*mut (), *mut T>(self.pointer.as_ref().pointers.data).as_mut().unwrap().trace()
            }
        }
    }
}

impl<T: GcTrace> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            transmute::<*mut (), *mut T>(self.pointer.as_ref().pointers.data).as_ref().unwrap()
        }
    }
}

impl<T: GcTrace> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            transmute::<*mut (), *mut T>(self.pointer.as_mut().pointers.data).as_mut().unwrap()
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
