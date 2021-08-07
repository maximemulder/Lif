use crate::runtime::gc::{ GcRef, GcTrace };

pub struct GcCache {
    caches: Vec<Vec<GcRef<()>>>,
}

impl GcCache {
    pub fn new() -> Self {
        Self {
            caches: vec![vec![]],
        }
    }

    pub fn push(&mut self) {
        self.caches.push(Vec::new());
    }

    pub fn pop(&mut self) {
        self.caches.pop();
    }

    pub fn store<T: GcTrace>(&mut self, r#ref: GcRef<T>) {
        self.insert(1, r#ref);
    }

    pub fn bubble<T: GcTrace>(&mut self, r#ref: GcRef<T>) {
        self.insert(2, r#ref);
    }

    fn insert<T: GcTrace>(&mut self, index: usize, r#ref: GcRef<T>) {
        let index = self.caches.len() - index;
        self.caches[index].push(r#ref.anonymize());
    }
}

impl GcTrace for GcCache {
    fn trace(&mut self) {
        for caches in self.caches.iter_mut() {
            for cache in caches.iter_mut() {
                cache.trace();
            }
        }
    }
}
