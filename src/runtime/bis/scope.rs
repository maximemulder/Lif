use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::{Value, Variable};
use crate::runtime::bis::data::{Ref, GcClass};

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub type GcScope<'a> = GcRef<Scope<'a>>;

pub struct Scope<'a> {
    pub parent: Option<GcScope<'a>>,
    variables: HashMap<Box<str>, Variable<'a>>,
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
    pub fn declare(&mut self, name: &str, class: GcClass<'a>) {
        self.variables.insert(Box::from(name), Variable::undefined(class));
    }

    pub fn set_value(&mut self, name: &str, class: GcClass<'a>, value: Value<'a>) {
        self.variables.insert(Box::from(name), Variable::value(class, value));
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
}

impl GcTrace for Scope<'_> {
    fn trace(&mut self) {
        if let Some(parent) = self.parent.as_mut() {
            parent.trace()
        }

        for variable in self.variables.values_mut() {
            variable.trace();
        }
    }
}
