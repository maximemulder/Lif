use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::value::Value;

use std::collections::HashMap;
use std::ops::Deref;

pub type GcClass<'a> = GcRef<Class<'a>>;

pub struct Class<'a> {
    pub name: Box<str>,
    parent: Option<GcClass<'a>>,
    methods: HashMap<Box<str>, Value<'a>>,
    statics: HashMap<Box<str>, Value<'a>>,
}

impl<'a> Class<'a> {
    pub fn new(
        name: &str,
        parent: Option<GcClass<'a>>,
        methods: HashMap<Box<str>, Value<'a>>,
        statics: HashMap<Box<str>, Value<'a>>,
    ) -> Self {
        Self { name: Box::from(name), parent, methods, statics }
    }

    pub fn add_method(&mut self, name: &str, method: Value<'a>) {
        self.methods.insert(Box::from(name), method);
    }

    pub fn get_method(&self, name: &str) -> Option<Value<'a>> {
        if let Some(method) = self.methods.get(name).copied() {
            Some(method)
        } else if let Some(parent) = self.parent {
            parent.get_method(name)
        } else {
            None
        }
    }

    pub fn add_static(&mut self, name: &str, r#static: Value<'a>) {
        self.statics.insert(Box::from(name), r#static);
    }

    pub fn get_static(&self, name: &str) -> Option<Value<'a>> {
        self.statics.get(name).copied()
    }

    pub fn isa(&self, class: GcClass<'a>) -> bool {
        if std::ptr::eq(self, class.deref()) {
            true
        } else if let Some(parent) = self.parent  {
            parent.isa(class)
        } else {
            false
        }
    }
}

impl GcTrace for Class<'_> {
    fn trace(&mut self) {
        if let Some(parent) = self.parent.as_mut() {
            parent.trace();
        }

        for method in self.methods.values_mut() {
            method.trace();
        }
    }
}
