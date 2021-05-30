use crate::runtime::data::Tag;
use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::constructors::GcConstructor;
use crate::runtime::value::GcValue;
use std::collections::HashMap;

pub struct Class<'a> {
    pub tag: Tag,
    scope: GcScope<'a>,
    pub constructor: Option<GcConstructor<'a>>,
    pub parent:  Option<GcValue<'a>>,
    pub statics: HashMap<Box<str>, GcReference<'a>>,
    pub methods: HashMap<Box<str>, GcValue<'a>>,
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

    pub fn get_method(&self, name: &str) -> Option<GcValue<'a>> {
        if let Some(&method) = self.methods.get(name) {
            return Some(method);
        }

        if let Some(parent) = self.parent {
            return parent.data_class().get_method(name);
        }

        None
    }

    pub fn scope(&self) -> GcScope<'a> {
        self.scope
    }
}

impl GcTrace for Class<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        if let Some(constructor) = self.constructor.as_mut() {
            constructor.trace();
        }

        if let Some(parent) = &mut self.parent {
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
