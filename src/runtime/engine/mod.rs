mod new;
mod primitive;
mod scope;

use crate::memory::{ Own, Ref };
use crate::parser::{ Code, Grammar };
use crate::runtime::environment::Environment;
use crate::runtime::gc::{ GC_THRESHOLD, Gc, GcCache, GcRef, GcTrace };
use crate::runtime::primitives::Class;
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::r#return::{ ReturnFlow, ReturnReference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::utilities::tag::Tagger;
use crate::runtime::value::Value;

use std::io::{ Read, Write };

pub struct Taggers {
    generics:  Tagger,
    classes:   Tagger,
    functions: Tagger,
}

impl Taggers {
    pub fn new() -> Self {
        Self {
            generics:  Tagger::new(),
            classes:   Tagger::new(),
            functions: Tagger::new(),
        }
    }
}

pub struct Engine<'a> {
    pub grammar:     &'a Grammar,
    pub input:       &'a mut dyn Read,
    pub output:      &'a mut dyn Write,
    pub error:       &'a mut dyn Write,
    pub environment: Environment<'a>,
    taggers:         Taggers,
    gc:              Gc,
    cache:           GcCache,
    codes:           Vec<Own<Code>>,
    frames:          Vec<GcScope<'a>>,
    scope:           GcScope<'a>,
    undefined:       GcReference<'a>,
}

impl<'a> Engine<'a> {
    pub fn new(grammar: &'a Grammar, input: &'a mut dyn Read, output: &'a mut dyn Write, error: &'a mut dyn Write) -> Self {
        let mut engine = Self {
            grammar,
            input,
            output,
            error,
            environment: Environment::new(),
            taggers:     Taggers::new(),
            gc:          Gc::new(),
            cache:       GcCache::new(),
            codes:       Vec::new(),
            frames:      Vec::new(),
            scope:       GcScope::null(),
            undefined:   GcReference::null(),
        };

        engine.scope = engine.alloc(Scope::new(None));
        engine.undefined = engine.alloc(Reference::new_constant(None));
        engine.populate();
        engine
    }
}

impl Engine<'_> {
    pub fn alloc<T: GcTrace>(&mut self, object: T) -> GcRef<T> {
        let r#ref = self.gc.alloc(object);
        self.cache.store(r#ref);
        r#ref
    }
}

impl<'a> Engine<'a> {
    pub fn new_reference(&mut self, value: Value<'a>) -> GcReference<'a> {
        self.alloc(Reference::new_variable(Some(value), self.environment.any))
    }

    pub fn new_variable(&mut self, value: Option<Value<'a>>, r#type: GcRef<Class<'a>>) -> GcReference<'a> {
        self.alloc(Reference::new_variable(value, r#type))
    }

    pub fn new_constant(&mut self, value: Value<'a>) -> GcReference<'a> {
        self.alloc(Reference::new_constant(Some(value)))
    }

    pub fn undefined(&mut self) -> GcReference<'a> {
        self.undefined
    }
}

impl<'a> Engine<'a> {
    pub fn set_variable(&mut self, name: &str, reference: GcReference<'a>) {
        self.scope.set_variable(name, reference);
    }

    pub fn get_variable(&self, name: &str) -> ReturnReference<'a> {
        self.scope.get_variable(name)
    }

    pub fn run(&mut self, code: Own<Code>) -> Option<GcReference<'a>> {
        self.codes.push(code);
        let node = Ref::new(self.codes.last().unwrap().walk_tree.as_ref().unwrap());
        let executable = Ref::as_ref(&node);
        match executable.get().walk(self) {
            Ok(reference) => Some(reference),
            Err(error) => {
                writeln!(self.error, "{}", error.get_message()).unwrap();
                None
            },
        }
    }

    pub fn run_gc(&mut self, callback: impl FnOnce(&mut Engine<'a>) -> ReturnFlow<'a>) -> ReturnFlow<'a> {
        self.cache.push();
        let r#return = callback(self);
        if let Ok(flow) = r#return.as_ref() {
            self.cache.bubble(flow.reference);
        }

        self.cache.pop();
        if self.gc.allocations() > GC_THRESHOLD {
            self.trace();
            self.gc.collect();
        }

        r#return
    }
}

impl GcTrace for Engine<'_> {
    fn trace(&mut self) {
        self.environment.trace();
        self.cache.trace();
        self.scope.trace();
        self.undefined.trace();
        for frame in self.frames.iter_mut() {
            frame.trace();
        }
    }
}
