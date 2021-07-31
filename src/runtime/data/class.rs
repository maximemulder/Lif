use crate::runtime::data::Generic;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::constructors::GcConstructor;
use crate::runtime::utilities::tag::Tag;
use crate::runtime::value::Value;

use std::collections::HashMap;
use std::ops::Deref;

pub struct Class<'a> {
    tag: Tag,
    pub scope: GcScope<'a>,
    constructor: Option<GcConstructor<'a>>,
    parent: Option<GcRef<Class<'a>>>,
    gc: bool,
    statics: HashMap<Box<str>, GcReference<'a>>,
    methods: HashMap<Box<str>, Value<'a>>,
}

impl<'a> Class<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parent: Option<GcRef<Class<'a>>>, gc: bool) -> Self {
        Self {
            tag,
            constructor: None,
            scope,
            parent,
            gc,
            statics: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn scope(&self) -> GcScope<'a> {
        self.scope
    }

    pub fn constructor(&self) -> Option<GcConstructor<'a>> {
        self.constructor
    }

    pub fn set_constructor(&mut self, constructor: GcConstructor<'a>) {
        debug_assert!(self.constructor.is_none());
        self.constructor = Some(constructor);
    }

    pub fn parent(&self) -> Option<GcRef<Class<'a>>> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: GcRef<Class<'a>>) {
        debug_assert!(self.parent.is_none());
        self.parent = Some(parent);
    }

    pub fn gc(&self) -> bool {
        self.gc
    }

    pub fn get_method(&self, name: &str) -> Option<Value<'a>> {
        if let Some(method) = self.methods.get(name).copied() {
            return Some(method);
        }

        if let Some(parent) = self.parent {
            return parent.get_method(name);
        }

        None
    }

    pub fn set_method(&mut self, name: &str, reference: Value<'a>) {
        self.methods.insert(Box::from(name), reference);
    }

    pub fn get_static(&self, name: &str) -> Option<GcReference<'a>> {
        self.statics.get(name).copied()
    }

    pub fn set_static(&mut self, name: &str, reference: GcReference<'a>) {
        self.statics.insert(Box::from(name), reference);
    }

    pub fn is(&self, class: GcRef<Class<'a>>) -> bool {
        if self as *const Class == class.deref() as *const Class {
            true
        } else if let Some(parent) = self.parent() {
            parent.is(class)
        } else {
            false
        }
    }

    pub fn is_generic(&self, generic: GcRef<Generic<'a>>) -> bool {
        if let Some(constructor) = self.constructor() {
            if constructor.generic == generic {
                return true;
            }
        }

        false
    }
}

impl GcTrace for Class<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        if let Some(constructor) = self.constructor.as_mut() {
            constructor.trace();
        }

        if let Some(parent) = self.parent.as_mut() {
            parent.trace();
        }

        for r#static in self.statics.values_mut() {
            r#static.trace();
        }

        for method in self.methods.values_mut() {
            method.trace();
        }
    }
}
