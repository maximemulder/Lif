use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::constructors::GcConstructor;
use crate::runtime::value::GcValue;
use std::collections::HashMap;

pub struct Class<'a> {
    tag: Tag,
    scope: GcScope<'a>,
    constructor: Option<GcConstructor<'a>>,
    parent: Option<GcValue<'a>>,
    statics: HashMap<Box<str>, GcReference<'a>>,
    methods: HashMap<Box<str>, GcValue<'a>>,
}

impl<'a> Class<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parent: Option<GcValue<'a>>) -> Self {
        Self {
            tag,
            constructor: None,
            scope,
            parent,
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

    pub fn parent(&self) -> Option<GcValue<'a>> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: GcValue<'a>) {
        debug_assert!(self.parent.is_none());
        self.parent = Some(parent);
    }

    pub fn get_method(&self, engine: &Engine<'a>, name: &str) -> Option<GcValue<'a>> {
        if let Some(method) = self.methods.get(name).copied() {
            return Some(method);
        }

        if let Some(parent) = self.parent {
            return parent.get_ref::<Class>(engine).get_method(engine, name);
        }

        None
    }

    pub fn set_method(&mut self, name: &str, reference: GcValue<'a>) {
        self.methods.insert(Box::from(name), reference);
    }

    pub fn get_static(&self, name: &str) -> Option<GcReference<'a>> {
        self.statics.get(name).copied()
    }

    pub fn set_static(&mut self, name: &str, reference: GcReference<'a>) {
        self.statics.insert(Box::from(name), reference);
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
