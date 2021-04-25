use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::value::GcValue;
use crate::runtime::utilities::Arguments;

pub type GcConstructor<'a> = GcRef<Constructor<'a>>;

pub struct Constructor<'a> {
    pub generic: GcValue<'a>,
    pub arguments: Arguments<'a>,
    pub value: GcValue<'a>,
}

impl<'a> Constructor<'a> {
    pub fn new(generic: GcValue<'a>, arguments: Arguments<'a>, value: GcValue<'a>) -> Self {
        Self {
            generic,
            arguments,
            value,
        }
    }

    fn test(&self, values: &[GcValue<'a>]) -> bool {
        if self.arguments.len() != values.len() {
            return false;
        }

        self.arguments.iter().copied().zip(values.iter().copied()).all(|(argument, value)| argument.is(value))
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

    pub fn record(&mut self, engine: &mut Engine<'a>, generic: GcValue<'a>, arguments: Arguments<'a>, value: GcValue<'a>) -> GcConstructor<'a> {
        let constructor = engine.alloc(Constructor::new(generic, arguments, value));
        self.constructors.push(constructor);
        constructor
    }

    pub fn get(&self, values: &[GcValue<'a>]) -> Option<GcValue<'a>> {
        for constructor in self.constructors.iter() {
            if constructor.test(values) {
                return Some(constructor.value);
            }
        }

        None
    }
}

impl GcTrace for Constructors<'_> {
    fn trace(&mut self) {
        for constructor in self.constructors.iter_mut() {
            constructor.trace();
        }
    }
}
