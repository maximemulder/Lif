use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::reference::GcReference;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;
use std::collections::HashMap;

pub type GcScope<'a> = GcRef<Scope<'a>>;

pub struct Scope<'a> {
    parent: Option<GcScope<'a>>,
    source: Option<GcValue<'a>>,
    variables: HashMap<Box<str>, GcReference<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<GcScope<'a>>) -> Self {
        Self {
            parent,
            source: None,
            variables: HashMap::new(),
        }
    }

    pub fn parent(&self) -> Option<GcScope<'a>> {
        self.parent
    }

    pub fn source(&self) -> Option<GcValue<'a>> {
        self.source
    }

    pub fn get_variable(&self, name: &str) -> ReturnReference<'a> {
        if let Some(value) = self.variables.get(name).copied() {
            Ok(value)
        } else if let Some(parent) = self.parent {
            parent.get_variable(name)
        } else {
            Err(Error::new_undeclared_variable(name))
        }
    }

    pub fn set_variable(&mut self, name: &str, reference: GcReference<'a>) {
        self.variables.insert(Box::from(name), reference);
    }

    pub fn set_source(&mut self, engine: &mut Engine<'a>, name: &str, source: GcValue<'a>) {
        debug_assert!(self.source.is_none());
        self.source = Some(source);
        let reference = engine.new_constant(source);
        self.set_variable(name, reference);
    }
}

impl GcTrace for Scope<'_> {
    fn trace(&mut self) {
        if let Some(parent) = self.parent.as_mut() {
            parent.trace();
        }

        if let Some(source) = self.source.as_mut() {
            source.trace();
        }

        for variable in self.variables.values_mut() {
            variable.trace();
        }
    }
}
