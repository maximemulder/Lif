use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::reference::GcReference;
use std::collections::HashMap;

pub type GcScope<'a> = GcRef<Scope<'a>>;

pub struct Scope<'a> {
    pub parent: Option<GcScope<'a>>,
    variables: HashMap<Box<str>, GcReference<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn new_child(scope: GcScope<'a>) -> Self {
        Self {
            parent: Some(scope),
            variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<GcReference<'a>> {
        self.variables.get(name).copied()
    }

    pub fn add_variable(&mut self, name: &str, reference: GcReference<'a>) {
        self.variables.insert(Box::from(name), reference);
    }
}

impl GcTrace for Scope<'_> {
    fn trace(&mut self) {
        if let Some(parent) = &mut self.parent {
            parent.trace();
        }

        for variable in self.variables.values_mut() {
            variable.trace();
        }
    }
}
