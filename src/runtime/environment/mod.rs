mod any;
mod array;
mod boolean;
mod class;
mod file;
mod float;
mod function;
mod generic;
mod integer;
mod method;
mod nullable;
mod object;
mod string;

use crate::parser::Code;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::{ Class, Generic, GenericPrimitive };
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;
use crate::walker::nodes::{ AExpression, AProgram };

use std::process;

pub struct Environment<'a> {
    pub any:                GcRef<Class<'a>>,
    pub array:              GcRef<Generic<'a>>,
    pub array_any:          GcRef<Class<'a>>,
    pub boolean:            GcRef<Class<'a>>,
    pub class:              GcRef<Class<'a>>,
    pub file:               GcRef<Class<'a>>,
    pub float:              GcRef<Class<'a>>,
    pub function:           GcRef<Class<'a>>,
    pub generic:            GcRef<Class<'a>>,
    pub method:             GcRef<Class<'a>>,
    pub nullable:           GcRef<Generic<'a>>,
    pub object:             GcRef<Class<'a>>,
    pub integer:            GcRef<Class<'a>>,
    pub string:             GcRef<Class<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            any:                GcRef::null(),
            array:              GcRef::null(),
            array_any:          GcRef::null(),
            boolean:            GcRef::null(),
            class:              GcRef::null(),
            file:               GcRef::null(),
            float:              GcRef::null(),
            function:           GcRef::null(),
            generic:            GcRef::null(),
            method:             GcRef::null(),
            nullable:           GcRef::null(),
            object:             GcRef::null(),
            integer:            GcRef::null(),
            string:             GcRef::null(),
        }
    }
}

impl GcTrace for Environment<'_> {
    fn trace(&mut self) {
        for class in [
            self.any,
            self.array_any,
            self.boolean,
            self.class,
            self.file,
            self.float,
            self.function,
            self.generic,
            self.integer,
            self.method,
            self.object,
            self.string
        ].iter_mut() {
            class.trace();
        }

        for generic in [self.array, self.nullable].iter_mut() {
            generic.trace();
        }
    }
}

impl<'a> Engine<'a> {
    pub fn populate(&mut self) {
        self.environment.class = self.primitive_origin_class("Class", None, true);
        self.environment.any   = self.primitive_class("Any", None, false);

        self.environment.class.set_parent(self.environment.any);

        self.environment.boolean  = self.primitive_class("Boolean",  Some(self.environment.any), false);
        self.environment.file     = self.primitive_class("File",     Some(self.environment.any), false);
        self.environment.float    = self.primitive_class("Float",    Some(self.environment.any), false);
        self.environment.function = self.primitive_class("Function", Some(self.environment.any), true);
        self.environment.generic  = self.primitive_class("Generic",  Some(self.environment.any), true);
        self.environment.integer  = self.primitive_class("Integer",  Some(self.environment.any), false);
        self.environment.method   = self.primitive_class("Method",   Some(self.environment.any), true);
        self.environment.object   = self.primitive_class("Object",   Some(self.environment.any), true);
        self.environment.string   = self.primitive_class("String",   Some(self.environment.any), true);

        self.environment.array    = self.primitive_generic("Array",  Box::new([Box::from("T")]), GenericPrimitive::new(&array::create));
        self.environment.nullable = self.primitive_generic("Option", Box::new([Box::from("T")]), GenericPrimitive::new(&nullable::create));

        self.environment.array_any = {
            let array = self.environment.array;
            array.clone().call(self, array, &mut [Value::primitive_gc(self, self.environment.any)]).ok().unwrap().get_value().get_gc::<Class>(self)
        };

        any::populate(self);
        array::populate(self);
        boolean::populate(self);
        class::populate(self);
        float::populate(self);
        function::populate(self);
        file::populate(self);
        generic::populate(self);
        integer::populate(self);
        method::populate(self);
        object::populate(self);
        nullable::populate(self);
        string::populate(self);

        let Environment { any, class, integer, string, .. } = self.environment;
        self.primitive_function("assert", [("value", any)], None, None, &assert);
        self.primitive_function("error", [("value", any)], None, None, &error);
        self.primitive_function("eval", [("code", string)], None, None, &eval);
        self.primitive_function("exec", [("code", string)], None, None, &exec);
        self.primitive_function("exit", [("code", integer)], None, None, &exit);
        self.primitive_function("include", [("file", string)], None, None, &include);
        self.primitive_function("new", [("class", class)], None, Some(any), &new);
        self.primitive_function("print", [("value", any)], None, None, &print);
    }
}

fn assert<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    if  arguments[0].get::<bool>(engine) {
        panic!();
    }

    Ok(engine.undefined())
}

fn error<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let string = arguments[0].call_fstr(engine)?;
    writeln!(engine.error, "{}", string).unwrap();
    Ok(engine.undefined())
}

fn eval<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let code = Code::from_string::<AExpression>(&engine.grammar, engine.grammar.expression, &arguments[0].get_gc::<String>(engine));
    Ok(match engine.run(code) {
        Some(reference) => reference,
        None => engine.undefined(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let code = Code::from_string::<AProgram>(&engine.grammar, engine.grammar.program, &arguments[0].get_gc::<String>(engine));
    engine.run(code);
    Ok(engine.undefined())
}

fn exit<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    process::exit(arguments[0].get::<isize>(engine) as i32);
}

fn include<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    engine.run_frame(engine.scope().parent().unwrap(), |engine| {
        let code = Code::from_file::<AProgram>(&engine.grammar, engine.grammar.program, &arguments[0].get_gc::<String>(engine)).unwrap();
        engine.run(code);
    });

    Ok(engine.undefined())
}

fn new<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_object(arguments[0].get_gc::<Class>(engine)))
}

fn print<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let string = arguments[0].call_fstr(engine)?;
    writeln!(engine.output, "{}", string).unwrap();
    Ok(engine.undefined())
}
