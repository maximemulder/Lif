use crate::memory::Mut;
use crate::runtime::gc::{ GcGuard, GcTrace };

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };

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

    pub fn new(guard: Mut<GcGuard>) -> Self {
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
        self.guard.cast_ref::<T>()
    }
}

impl<T: GcTrace> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.cast_mut::<T>()
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
