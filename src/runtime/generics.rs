use crate::runtime::Value;
use crate::runtime::data::{GcClass, GcGeneric};

pub struct Generics<'a> {
    entries: Vec<(GcGeneric<'a>, Box<[GcClass<'a>]>, Value<'a>)>,
}

impl<'a> Generics<'a> {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn get(&self, generic: GcGeneric<'a>, args: &[GcClass<'a>]) -> Option<Value<'a>> {
        self.entries.iter()
            .find(|entry| entry.0 == generic && entry.1.iter().eq(args.iter()))
            .map(|entry| entry.2)
    }

    pub fn save(&mut self, generic: GcGeneric<'a>, args: Box<[GcClass<'a>]>, value: Value<'a>) {
        self.entries.push((generic, args, value));
    }
}
