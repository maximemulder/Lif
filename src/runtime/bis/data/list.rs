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
}

impl GcTrace for List<'_> {
    fn trace(&mut self) {
        for element in self.0.iter_mut() {
            element.trace()
        }
    }
}
