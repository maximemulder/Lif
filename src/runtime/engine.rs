use crate::ast::Pos;
use crate::memory::Own;
use crate::parser::{Ast, Code, Grammar};
use crate::runtime::{Env, Value};
use crate::runtime::gc::{Gc, GcCache, GcRef, GcTrace, GC_THRESHOLD};
use crate::runtime::data::{Data, Class, Function, Generic, List, Method, Object, Ref, String, GcClass, GcGeneric};
use crate::runtime::flow::{Res, ResValue};
use crate::runtime::generics::Generics;
use crate::runtime::primitive::populate;
use crate::runtime::scope::{Scope, GcScope};
use crate::runtime::eval::errors::*;

use std::io::{Write, Read};

use super::flow::Flow;
use super::frame::Frame;

pub struct Io<'a> {
    pub r#in: &'a mut dyn Read,
    pub out: &'a mut dyn Write,
    pub err: &'a mut dyn Write,
}

impl<'a> Io<'a> {
    pub fn new(r#in: &'a mut dyn Read, out: &'a mut dyn Write, err: &'a mut dyn Write) -> Self {
        Self { r#in, out, err }
    }
}

pub struct Engine<'a> {
    pub io: Io<'a>,
    pub env: Env<'a>,
    pub frames: Vec<Frame<'a>>,
    pub scope: GcScope<'a>,
    pub grammar: &'a Grammar,
    pub codes: Vec<Own<Code>>,
    gc: Gc,
    cache: GcCache,
    generics: Generics<'a>,
}

impl<'a> Engine<'a> {
    pub fn new(io: Io<'a>, grammar: &'a Grammar) -> Self {
        let mut engine = Self {
            io,
            grammar,
            env: Env::new(),
            frames: Vec::new(),
            scope: GcScope::null(),
            codes: Vec::new(),
            gc: Gc::new(),
            cache: GcCache::new(),
            generics: Generics::new(),
        };

        engine.scope = engine.alloc(Scope::new(None));
        populate(&mut engine);
        engine
    }

    pub fn run(&mut self, code: Own<Code>) -> Option<Value<'a>> {
        match self.run_inner(code) {
            Ok(value) => Some(value),
            Err(error) => {
                writeln!(self.io.err, "{}", error.get_message()).unwrap();
                writeln!(self.io.err, "STACK TRACE:").unwrap();
                for frame in self.frames.iter().rev() {
                    writeln!(self.io.err, "  in `{}` {}", frame.name(), frame.pos().print_pos()).unwrap();
                }

                None
            },
        }
    }

    fn run_inner(&mut self, code: Own<Code>) -> ResValue<'a> {
        self.with_frame(Frame::new_main(Pos::DUMMY, self.scope), |engine| {
            let result = match code.abstract_tree.as_ref().unwrap() {
                Ast::Program(program) => program.eval(engine),
                Ast::Expression(expr) => expr.eval(engine),
            };

            engine.codes.push(code);
            match result? {
                Flow::None(value) => Ok(value),
                Flow::Jump(jump) => error_jump(jump),
            }
        })
    }
}

impl<'a> Engine<'a> {
    pub fn with_scope<T>(&mut self, f: impl FnOnce(&mut Self) -> Res<T>) -> Res<T> {
        let parent = self.scope;
        let child = self.new_scope(parent);
        self.scope = child;
        let result = f(self)?;
        self.scope = parent;
        Ok(result)
    }

    pub fn with_frame<T>(&mut self, frame: Frame<'a>, f: impl FnOnce(&mut Engine<'a>) -> Res<T>) -> Res<T> {
        let scope = self.scope;
        self.scope = frame.scope();
        self.frames.push(frame);
        let result = self.with_scope(f)?;
        self.frames.pop().unwrap();
        self.scope = scope;
        Ok(result)
    }

    fn new_scope(&mut self, scope: GcScope<'a>) -> GcScope<'a> {
        self.alloc(Scope::new(Some(scope)))
    }
}

impl<'a> Engine<'a> {

    pub fn declare(&mut self, name: &str, class: GcClass<'a>) {
        self.scope.declare(name, class);
    }

    pub fn write(&mut self, name: &str, class: GcClass<'a>, value: Value<'a>) {
        self.scope.set_value(name, class, value);
    }

    pub fn write_value(&mut self, name: &str, value: Value<'a>) {
        self.write(name, value.class, value);
    }

    pub fn read(&mut self, pos: Pos, name: &str) -> Res<Ref<'a>> {
        match self.scope.get_ref(name) {
            Some(r#ref) => Ok(r#ref),
            None => error_undeclared(pos, name),
        }
    }

    pub fn frame(&self) -> &Frame<'a> {
        self.frames.last().unwrap()
    }

    pub fn get_generic(&mut self, pos: Pos, generic: GcGeneric<'a>, args: Box<[GcClass<'a>]>) -> ResValue<'a> {
        if let Some(value) = self.generics.get(generic, &args) {
            return Ok(value);
        }

        let value = generic.apply(self, pos, &args)?;
        self.generics.save(generic, args, value);
        Ok(value)
    }
}

impl<'a> Engine<'a> {
    pub fn alloc<T: GcTrace>(&mut self, object: T) -> GcRef<T> {
        self.gc.alloc(object)
    }

    pub fn with_gc<T: GcTrace>(&mut self, f: impl FnOnce(&mut Self) -> Res<GcRef<T>>) -> Res<GcRef<T>> {
        if self.gc.allocations() > GC_THRESHOLD {
            self.trace();
            self.gc.collect();
        }

        self.cache.push();
        let ret = f(self);
        if let Ok(object) = ret {
            self.cache.bubble(object);
        }

        self.cache.pop();
        ret
    }
}

impl<'a> Engine<'a> {
    pub fn new_bool(&mut self, bool: bool) -> Value<'a> {
        Value::new(self.env.bool, Data::Bool(bool))
    }

    pub fn new_class(&mut self, class: Class<'a>) -> Value<'a> {
        Value::new(self.env.class, Data::Class(self.alloc(class)))
    }

    pub fn new_class_primitive(&mut self, class: GcClass<'a>) -> Value<'a> {
        Value::new(self.env.class, Data::Class(class))
    }

    pub fn new_float(&mut self, float: f64) -> Value<'a> {
        Value::new(self.env.float, Data::Float(float))
    }

    pub fn new_function(&mut self, function: Function<'a>) -> Value<'a> {
        Value::new(self.env.function, Data::Function(self.alloc(function)))
    }

    pub fn new_generic(&mut self, generic: Generic<'a>) -> Value<'a> {
        Value::new(self.env.generic, Data::Generic(self.alloc(generic)))
    }

    pub fn new_generic_primitive(&mut self, generic: GcGeneric<'a>) -> Value<'a> {
        Value::new(self.env.generic, Data::Generic(generic))
    }

    pub fn new_int(&mut self, int: i64) -> Value<'a> {
        Value::new(self.env.int, Data::Int(int))
    }

    pub fn new_list(&mut self, values: &[Value<'a>]) -> Value<'a> {
        Value::new(self.env.list_any, Data::List(self.alloc(List::new(self.env.any, values))))
    }

    pub fn new_method(&mut self, receiver: Value<'a>, function: Value<'a>) -> Value<'a> {
        Value::new(self.env.method, Data::Method(self.alloc(Method::new(receiver, function))))
    }

    pub fn new_object(&mut self, class: GcClass<'a>) -> Value<'a> {
        Value::new(class, Data::Object(self.alloc(Object::new())))
    }

    pub fn new_ref(&mut self, r#ref: Ref<'a>) -> Value<'a> {
        Value::new(self.env.r#ref, Data::Ref(r#ref))
    }

    pub fn new_string(&mut self, string: &str) -> Value<'a> {
        Value::new(self.env.string, Data::String(self.alloc(String(Box::from(string)))))
    }

    pub fn new_void(&mut self) -> Value<'a> {
        Value::new(self.env.void, Data::Void(()))
    }
}

impl GcTrace for Engine<'_> {
    fn trace(&mut self) {
        self.cache.trace();
        self.env.trace();
        self.scope.trace();
        for frame in self.frames.iter_mut() {
            frame.trace();
        }
    }
}
