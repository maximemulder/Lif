use crate::memory::Mut;
use crate::runtime::gc::{ GcGuard, GcRef, GcTrace };

pub struct Registries {
    registries: Vec<Vec<Mut<GcGuard>>>,
}

impl Registries {
    pub fn new() -> Self {
        Self {
            registries: vec![Vec::new()],
        }
    }

    pub fn push(&mut self) {
        self.registries.push(Vec::new());
    }

    pub fn pop(&mut self) {
        self.registries.pop();
    }

    pub fn store<T: GcTrace>(&mut self, r#ref: GcRef<T>) {
        self.enter(r#ref, 1);
    }

    pub fn cache<T: GcTrace>(&mut self, r#ref: GcRef<T>) {
        self.enter(r#ref, 2);
    }

    fn enter<T: GcTrace>(&mut self, r#ref: GcRef<T>, index: usize) {
        let index = self.registries.len() - index;
        self.registries[index].push(r#ref.get_guard());
    }
}

impl GcTrace for Registries {
    fn trace(&mut self) {
        for registries in self.registries.iter_mut() {
            for registry in registries.iter_mut() {
                registry.trace();
            }
        }
    }
}
