use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::{Value, ValueRef};
use crate::runtime::bis::data::Ref;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub type GcScope<'a> = GcRef<Scope<'a>>;

pub struct Scope<'a> {
    pub parent: Option<GcScope<'a>>,
    variables: HashMap<Box<str>, Option<Value<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<GcScope<'a>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
        }
    }
}

impl<'a> Scope<'a> {
    pub fn get_value(&self, name: &str) -> ValueRef<'a> {
        if let Some(variable) = self.variables.get(name).copied() {
            match variable {
                Some(value) => ValueRef::Value(value),
                None        => ValueRef::Undefined,
            }
        } else if let Some(parent) = self.parent {
            parent.get_value(name)
        } else {
            ValueRef::Undeclared
        }
    }

    pub fn get_ref(&mut self, name: &str) -> Option<Ref<'a>> {
        if let Entry::Occupied(mut entry) = self.variables.entry(name.into()) {
            Some(Ref::new(entry.get_mut()))
        } else if let Some(mut parent) = self.parent {
            parent.get_ref(name)
        } else {
            None
        }
    }

    pub fn set_value(&mut self, name: &str, value: Value<'a>) {
        self.variables.insert(Box::from(name), Some(value));
    }
}

impl GcTrace for Scope<'_> {
    fn trace(&mut self) {
        if let Some(parent) = self.parent.as_mut() {
            parent.trace()
        }

        for value in self.variables.values_mut() {
            if let Some(value) = value.as_mut() {
                value.trace();
            }
        }
    }
}
