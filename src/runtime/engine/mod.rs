mod control;
mod new;

use crate::code::Code;
use crate::memory::{ Own, Ref };
use crate::nodes::Executable;
use crate::parser::Parser;
use crate::runtime::ReturnReference;
use crate::runtime::data::{ Data, Tagger };
use crate::runtime::primitives::Primitives;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GC_THRESHOLD, Gc, GcTrace };
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::value::{ GcValue, Value };

use std::cmp::min;
use std::io::{ Read, Write };

#[derive(PartialEq, Eq)]
pub enum Control {
    Return,
    Break,
    Continue,
}

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
    pub parser:     &'a Parser,
    pub input:      &'a mut dyn Read,
    pub output:     &'a mut dyn Write,
    pub error:      &'a mut dyn Write,
    pub primitives: Primitives<'a>,
    taggers:        Taggers,
    gc:             Gc,
    codes:          Vec<Own<Code>>,
    registries:     Vec<Vec<GcReference<'a>>>,
    frames:         Vec<GcScope<'a>>,
    scope:          GcScope<'a>,
    undefined:      GcReference<'a>,
    control:        Option<Control>,
    allocations:    usize,
}

impl<'a> Engine<'a> {
    pub fn new(parser: &'a Parser, input: &'a mut dyn Read, output: &'a mut dyn Write, error: &'a mut dyn Write) -> Self {
        let mut engine = Self {
            parser,
            input,
            output,
            error,
            primitives:  Primitives::new(),
            taggers:     Taggers::new(),
            gc:          Gc::new(),
            codes:       Vec::new(),
            registries:  Vec::new(),
            frames:      Vec::new(),
            scope:       GcScope::null(),
            undefined:   GcReference::null(),
            control:     None,
            allocations: 0,
        };

        engine.scope = engine.alloc_scope(Scope::new());
        engine.undefined = engine.alloc_reference(Reference::new_constant(None));
        engine.registries.push(Vec::new());
        engine.populate();
        engine
    }
}

impl<'a> Engine<'a> {
    pub fn alloc_value(&mut self, value: Value<'a>) -> GcValue<'a> {
        let value = self.gc.alloc(value);
        self.allocations += 1;
        value
    }

    pub fn alloc_reference(&mut self, reference: Reference<'a>) -> GcReference<'a> {
        let reference = self.gc.alloc(reference);
        self.allocations += 1;
        reference
    }

    pub fn alloc_scope(&mut self, scope: Scope<'a>) -> GcScope<'a> {
        let scope = self.gc.alloc(scope);
        self.allocations += 1;
        scope
    }
}

impl<'a> Engine<'a> {
    pub fn new_value(&mut self, class: GcValue<'a>, data: Data<'a>) -> GcValue<'a> {
        self.alloc_value(Value::new(class, data))
    }

    pub fn new_reference(&mut self, value: GcValue<'a>) -> GcReference<'a> {
        self.alloc_reference(Reference::new_variable(Some(value), self.primitives.any))
    }

    pub fn new_variable(&mut self, value: Option<GcValue<'a>>, r#type: GcValue<'a>) -> GcReference<'a> {
        self.alloc_reference(Reference::new_variable(value, r#type))
    }

    pub fn new_constant(&mut self, value: GcValue<'a>) -> GcReference<'a> {
        self.alloc_reference(Reference::new_constant(Some(value)))
    }

    pub fn undefined(&mut self) -> GcReference<'a> {
        self.undefined
    }
}

impl<'a> Engine<'a> {
    pub fn push_scope(&mut self) {
        self.scope = self.alloc_scope(Scope::new_child(self.scope));
    }

    pub fn pop_scope(&mut self) {
        self.scope = self.scope.parent.unwrap();
    }

    pub fn push_frame(&mut self, frame: GcScope<'a>) {
        self.frames.push(self.scope);
        self.scope = frame;
    }

    pub fn pop_frame(&mut self) {
        self.scope = self.frames.pop().unwrap();
    }
}

impl<'a> Engine<'a> {
    pub fn add_variable(&mut self, name: &str, reference: GcReference<'a>) {
        self.scope.add_variable(name, reference);
    }

    pub fn get_variable(&self, name: &str) -> ReturnReference<'a> {
        let mut scope = self.scope;
        loop {
            if let Some(object) = scope.get_variable(name) {
                return Ok(object);
            }

            if let Some(parent) = scope.parent {
                scope = parent;
            } else {
                return Err(Error::new_undeclared_variable(name));
            }
        }
    }

    pub fn collect(&mut self) {
        self.trace();
        self.gc.collect();
        self.allocations = 0;
    }

    pub fn execute(&mut self, node: &dyn Executable) -> ReturnReference<'a> {
        self.registries.push(Vec::new());
        let reference = match node.execute(self) {
            Ok(reference) => reference,
            Err(error) => return Err(error),
        };

        let index = self.registries.len() - 2;
        self.registries[index].push(reference);
        self.registries.pop();
        /* if self.allocations > GC_THRESHOLD {
            self.collect();
        } */

        Ok(reference)
    }

    pub fn run(&mut self, code: Code) {
        use crate::nodes::build::program;
        let mut own = Own::new(code);
        if let Some(ast) = self.parser.parse(own.get_ref()) {
            own.ast = Some(ast);
            own.cst = Some(program(Ref::from_ref(&own.ast.as_ref().unwrap())));
            self.codes.push(own);
            let node = Ref::from_ref(self.codes.last().unwrap().cst.as_ref().unwrap());
            let executable = Ref::as_ref(&node);
            let result = self.execute(executable);
            if let Err(error) = result {
                let mut message = String::new();
                message += &error.message;
                if let Some(node) = error.node {
                    let code = node.code;
                    if let Some(name) = &code.name {
                        message += name;
                    }

                    message += "\n";
                    message += code.node_line(&node);
                    message += "\n";
                    message += &" ".repeat(code.node_shift_left(&node));
                    message += &"^".repeat(min(code.node_str(&node).len(), code.node_shift_right(&node)));
                    writeln!(self.error, "{}", message).unwrap();
                }
            }
        }
    }
}

impl GcTrace for Engine<'_> {
    fn trace(&mut self) {
        self.primitives.trace();
        self.scope.trace();
        self.undefined.trace();
        for registries in self.registries.iter_mut() {
            for registry in registries.iter_mut() {
                registry.trace();
            }
        }

        for frame in self.frames.iter_mut() {
            frame.trace();
        }
    }
}
