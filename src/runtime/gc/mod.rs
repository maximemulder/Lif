mod cache;
mod guard;
mod r#ref;
mod trace;

pub use cache::GcCache;
pub use r#ref::GcRef;
pub use trace::GcTrace;

use crate::memory::Own;
use guard::GcGuard;

pub const GC_THRESHOLD: usize = 0;

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
        self.guards.drain_filter(|guard| guard.reset());
        self.allocations = 0;
    }

    pub fn allocations(&self) -> usize {
        self.allocations
    }
}
