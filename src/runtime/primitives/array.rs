use crate::runtime::data::PrimitiveGeneric;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::Generic;
use crate::runtime::reference::GcReference;

pub struct Array<'a> {
    elements: Vec<GcReference<'a>>,
}

impl<'a> Array<'a> {
    pub fn new(elements: Vec<GcReference<'a>>) -> Self {
        Self {
            elements
        }
    }

    pub fn elements(&self) -> &[GcReference<'a>] {
        &self.elements
    }

    pub fn get(&self, index: usize) -> GcReference<'a> {
        self.elements[index]
    }

    pub fn append(&mut self, reference: GcReference<'a>) {
        self.elements.push(reference);
    }

    pub fn prepend(&mut self, reference: GcReference<'a>) {
        self.elements.insert(0, reference);
    }

    pub fn insert(&mut self, index: usize, reference: GcReference<'a>) {
        self.elements.insert(index, reference);
    }

    pub fn remove(&mut self, index: usize) {
        self.elements.remove(index);
    }
}

impl<'a> PrimitiveGeneric<'a> for Array<'a> {
    fn get_generic(engine: &Engine<'a>) -> GcRef<Generic<'a>> {
        engine.environment.array
    }
}

impl GcTrace for Array<'_> {
    fn trace(&mut self) {
        for element in self.elements.iter_mut() {
            element.trace();
        }
    }
}
