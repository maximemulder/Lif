use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::value::Value;

use std::iter::Copied;
use std::slice::Iter;

pub type GcList<'a> = GcRef<List<'a>>;

pub struct List<'a>(pub Vec<Value<'a>>);

impl<'a> List<'a> {
    pub fn iter(&self) -> Copied<Iter<'_, Value<'a>>> {
        self.0.iter().copied()
    }

    pub fn get(&self, index: usize) -> Value<'a> {
        self.0[index]
    }

    pub fn insert(&mut self, index: usize, value: Value<'a>) {
        self.0.insert(index, value);
    }

    pub fn append(&mut self, value: Value<'a>) {
        self.0.push(value);
    }

    pub fn prepend(&mut self, value: Value<'a>) {
        self.0.insert(0, value);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl GcTrace for List<'_> {
    fn trace(&mut self) {
        for element in self.0.iter_mut() {
            element.trace()
        }
    }
}
