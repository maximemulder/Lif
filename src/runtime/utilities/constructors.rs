use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::Generic;
use crate::runtime::value::Value;

pub type GcConstructor<'a> = GcRef<Constructor<'a>>;

pub struct Constructor<'a> {
    pub generic: GcRef<Generic<'a>>,
    pub arguments: Box<[Value<'a>]>,
    pub value: Value<'a>,
}

impl<'a> Constructor<'a> {
    pub fn new(generic: GcRef<Generic<'a>>, arguments: Box<[Value<'a>]>, value: Value<'a>) -> Self {
        Self {
            generic,
            arguments,
            value,
        }
    }

    fn check(&self, values: &mut [Value<'a>]) -> bool {
        if self.arguments.len() != values.len() {
            return false;
        }

        self.arguments.iter().copied().zip(values.iter().copied()).all(|(argument, value)| argument == value)
    }
}

impl GcTrace for Constructor<'_> {
    fn trace(&mut self) {
        self.generic.trace();
        self.value.trace();
        for argument in self.arguments.iter_mut() {
            argument.trace();
        }
    }
}

pub struct Constructors<'a> {
    constructors: Vec<GcConstructor<'a>>,
}

impl<'a> Constructors<'a> {
    pub fn new() -> Self {
        Self {
            constructors: Vec::new(),
        }
    }

    pub fn record(&mut self, engine: &mut Engine<'a>, generic: GcRef<Generic<'a>>, arguments: Box<[Value<'a>]>, value: Value<'a>) -> GcConstructor<'a> {
        let constructor = engine.alloc(Constructor::new(generic, arguments, value));
        self.constructors.push(constructor);
        constructor
    }

    pub fn get(&self, values: &mut [Value<'a>]) -> Option<Value<'a>> {
        Some(self.constructors.iter().find(|constructor| constructor.check(values))?.value)
    }
}

impl GcTrace for Constructors<'_> {
    fn trace(&mut self) {
        for constructor in self.constructors.iter_mut() {
            constructor.trace();
        }
    }
}
