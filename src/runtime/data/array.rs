use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;

pub struct Array<'a> {
    pub elements: Vec<GcReference<'a>>,
}

impl<'a> Array<'a> {
    pub fn new(elements: Vec<GcReference<'a>>) -> Self {
        Self {
            elements
        }
    }

    /*pub fn get(&self, index: usize) -> GcReference<'a> {
        self.elements[index]
    }

    pub fn push(&mut self, engine: &mut Engine<'a>, value: GcValue<'a>) {
        self.elements.push(engine.new_variable(Some(value), engine.primitives.any));
    }

    pub fn insert(&mut self, engine: &mut Engine<'a>, index: usize, value: GcValue<'a>) {
        self.elements.insert(index, engine.new_variable(Some(value), engine.primitives.any));
    }

    pub fn remove(&mut self, index: usize) {
        self.elements.remove(index);
    }

    pub fn slice(&self) -> &[GcReference<'a>] {
        &self.elements
    } */
}

impl GcTrace for Array<'_> {
    fn trace(&mut self) {
        for element in self.elements.iter_mut() {
            element.trace();
        }
    }
}
