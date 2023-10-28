use crate::ast::Pos;
use crate::memory::Own;
use crate::parser::{Ast, Code, Grammar};
use crate::runtime::bis::{Value, ValueRef};
use crate::runtime::bis::env::Env;
use crate::runtime::bis::error::Error;
use crate::runtime::gc::{Gc, GcCache, GcRef, GcTrace, GC_THRESHOLD};
use crate::runtime::bis::data::{Data, Class, Function, Generic, List, Method, Object, Ref, String, GcClass, GcGeneric};
use crate::runtime::bis::flow::{Res, ResValue};
use crate::runtime::bis::primitive::populate;
use crate::runtime::bis::scope::{Scope, GcScope};

use std::io::{Write, Read};

use super::flow::Flow;

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
    pub frames: Vec<GcScope<'a>>,
    pub grammar: &'a Grammar,
    pub codes: Vec<Own<Code>>,
    gc: Gc,
    cache: GcCache,
}

impl<'a> Engine<'a> {
    pub fn new(io: Io<'a>, grammar: &'a Grammar) -> Self {
        let mut engine = Self {
            io,
            grammar,
            env: Env::new(),
            frames: Vec::new(),
            codes: Vec::new(),
            gc: Gc::new(),
            cache: GcCache::new(),
        };

        let scope = engine.alloc(Scope::new(None));
        engine.frames.push(scope);
        populate(&mut engine);
        engine
    }

    pub fn run(&mut self, code: Own<Code>) -> Option<Value<'a>> {
        let result = match code.abstract_tree.as_ref().unwrap() {
            Ast::Program(program) => program.eval(self),
            Ast::Expression(expr) => expr.eval(self),
        };

        self.codes.push(code);
        match result {
            Ok(Flow::Value(value)) => Some(value),
            Ok(_) => panic!(),
            Err(error) => {
                writeln!(self.io.err, "{}", error.get_message()).unwrap();
                None
            },
        }
    }
}

impl<'a> Engine<'a> {
    pub fn get_scope(&self) -> GcScope<'a> {
        self.frames.last().unwrap().clone()
    }

    pub fn with_scope<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        self.swap_scope(|engine, scope| engine.new_scope(scope));
        let result = f(self);
        self.swap_scope(|_, scope| scope.parent.unwrap());
        result
    }

    pub fn with_frame<T>(&mut self, parent: GcScope<'a>, f: impl FnOnce(&mut Engine<'a>) -> T) -> T {
        let frame = self.new_scope(parent);
        self.frames.push(frame);
        let result = f(self);
        self.frames.pop().unwrap();
        result
    }

    fn new_scope(&mut self, scope: GcScope<'a>) -> GcScope<'a> {
        self.alloc(Scope::new(Some(scope)))
    }

    fn swap_scope(&mut self, f: impl FnOnce(&mut Self, GcScope<'a>) -> GcScope<'a>) {
        let scope = f(self, self.get_scope());
        *self.frames.last_mut().unwrap() = scope;
    }
}

impl<'a> Engine<'a> {
    pub fn read(&self, pos: Pos, name: &str) -> ResValue<'a> {
        match self.frames.last().unwrap().get_value(name) {
            ValueRef::Value(value) => Ok(value),
            ValueRef::Undeclared => Err(error_undeclared(pos, name)),
            ValueRef::Undefined  => Err(error_undefined(pos, name)),
        }
    }

    pub fn read_ref(&mut self, pos: Pos, name: &str) -> ResValue<'a> {
        match self.frames.last_mut().unwrap().get_ref(name) {
            Some(r#ref) => Ok(self.new_ref(r#ref)),
            None => Err(error_undeclared(pos, name)),
        }
    }

    pub fn write(&mut self, name: &str, value: Value<'a>) {
        self.frames.last_mut().unwrap().set_value(name, value)
    }
}

impl<'a> Engine<'a> {
    pub fn alloc<T: GcTrace>(&mut self, object: T) -> GcRef<T> {
        if self.gc.allocations() > GC_THRESHOLD {
            self.trace();
            self.gc.collect();
        }

        self.gc.alloc(object)
    }

    pub fn with_gc<T: GcTrace>(&mut self, f: impl FnOnce(&mut Self) -> Res<GcRef<T>>) -> Res<GcRef<T>> {
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

    pub fn new_list(&mut self, list: Vec<Value<'a>>) -> Value<'a> {
        Value::new(self.env.list_any, Data::List(self.alloc(List(list))))
    }

    pub fn new_method(&mut self, receiver: Value<'a>, function: Value<'a>) -> Value<'a> {
        Value::new(self.env.method, Data::Method(self.alloc(Method::new(receiver, function))))
    }

    pub fn new_object(&mut self, class: GcClass<'a>) -> Value<'a> {
        Value::new(class, Data::Object(self.alloc(Object::new())))
    }

    pub fn new_ref(&mut self, r#ref: Ref<'a>) -> Value<'a> {
        Value::new(self.env.any, Data::Ref(r#ref))
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
        for frame in self.frames.iter_mut() {
            frame.trace();
        }
    }
}

fn error_undeclared(pos: Pos, name: &str) -> Error {
    Error::new(pos, &format!("undeclared variable `{name}`"))
}

fn error_undefined(pos: Pos, name: &str) -> Error {
    Error::new(pos, &format!("undefined variable `{name}`"))
}
