use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::value::GcValue;

pub type GcConstructor<'a> = GcRef<Constructor<'a>>;

pub struct Constructor<'a> {
    pub generic: GcValue<'a>,
    pub arguments: Box<[GcValue<'a>]>,
    pub value: GcValue<'a>,
}

impl<'a> Constructor<'a> {
    pub fn new(generic: GcValue<'a>, arguments: Box<[GcValue<'a>]>, value: GcValue<'a>) -> Self {
        Self {
            generic,
            arguments,
            value,
        }
    }

    fn check(&self, engine: &Engine<'a>, values: &mut [GcValue<'a>]) -> bool {
        if self.arguments.len() != values.len() {
            return false;
        }

        self.arguments.iter().copied().zip(values.iter().copied()).all(|(argument, value)| argument.is(engine, value))
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

    pub fn record(&mut self, engine: &mut Engine<'a>, generic: GcValue<'a>, arguments: Box<[GcValue<'a>]>, value: GcValue<'a>) -> GcConstructor<'a> {
        let constructor = engine.alloc(Constructor::new(generic, arguments, value));
        self.constructors.push(constructor);
        constructor
    }

    pub fn get(&self, engine: &Engine<'a>, values: &mut [GcValue<'a>]) -> Option<GcValue<'a>> {
        Some(self.constructors.iter().find(|constructor| constructor.check(engine, values))?.value)
    }
}

impl GcTrace for Constructors<'_> {
    fn trace(&mut self) {
        for constructor in self.constructors.iter_mut() {
            constructor.trace();
        }
    }
}
