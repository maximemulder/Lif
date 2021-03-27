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

use crate::code::Code;
use crate::nodes::build;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

use std::process;

pub struct Primitives<'a> {
    pub any:                GcValue<'a>,
    pub array:              GcValue<'a>,
    pub array_any:          GcValue<'a>,
    pub boolean:            GcValue<'a>,
    pub class:              GcValue<'a>,
    pub file:               GcValue<'a>,
    pub float:              GcValue<'a>,
    pub function:           GcValue<'a>,
    pub function_code:      GcValue<'a>,
    pub function_primitive: GcValue<'a>,
    pub generic:            GcValue<'a>,
    pub generic_code:       GcValue<'a>,
    pub generic_primitive:  GcValue<'a>,
    pub method:             GcValue<'a>,
    pub nullable:           GcValue<'a>,
    pub object:             GcValue<'a>,
    pub integer:            GcValue<'a>,
    pub string:             GcValue<'a>,
}

impl<'a> Primitives<'a> {
    pub fn new() -> Self {
        Self {
            any:                GcValue::null(),
            array:              GcValue::null(),
            array_any:          GcValue::null(),
            boolean:            GcValue::null(),
            class:              GcValue::null(),
            file:               GcValue::null(),
            float:              GcValue::null(),
            function:           GcValue::null(),
            function_code:      GcValue::null(),
            function_primitive: GcValue::null(),
            generic:            GcValue::null(),
            generic_code:       GcValue::null(),
            generic_primitive:  GcValue::null(),
            method:             GcValue::null(),
            nullable:           GcValue::null(),
            object:             GcValue::null(),
            integer:            GcValue::null(),
            string:             GcValue::null(),
        }
    }
}

impl GcTrace for Primitives<'_> {
    fn trace(&mut self) {
        for class in [
            self.any,
            self.array,
            self.array_any,
            self.boolean,
            self.class,
            self.file,
            self.float,
            self.function,
            self.function_code,
            self.function_primitive,
            self.generic,
            self.integer,
            self.method,
            self.nullable,
            self.object,
            self.string
        ].iter_mut() {
            class.trace();
        }
    }
}

impl<'a> Engine<'a> {
    pub fn populate(&mut self) {
        self.primitives.class = self.new_class_primitive_value(None, "Class");
        self.primitives.any   = self.new_class_primitive_value(None, "Any");

        self.primitives.class.class = self.primitives.class;
        self.primitives.class.data_class_mut().parent = Some(self.primitives.any);
        self.primitives.any.data_class_mut().parent = None;

        self.primitives.boolean            = self.new_class_primitive_value(Some(self.primitives.any),      "Boolean");
        self.primitives.file               = self.new_class_primitive_value(Some(self.primitives.any),      "File");
        self.primitives.float              = self.new_class_primitive_value(Some(self.primitives.any),      "Float");
        self.primitives.function           = self.new_class_primitive_value(Some(self.primitives.any),      "Function");
        self.primitives.function_code      = self.new_class_primitive_value(Some(self.primitives.function), "FunctionCode");
        self.primitives.function_primitive = self.new_class_primitive_value(Some(self.primitives.function), "FunctionPrimitive");
        self.primitives.generic            = self.new_class_primitive_value(Some(self.primitives.any),      "Generic");
        self.primitives.generic_code       = self.new_class_primitive_value(Some(self.primitives.generic),  "GenericCode");
        self.primitives.generic_primitive  = self.new_class_primitive_value(Some(self.primitives.generic),  "GenericPrimitive");
        self.primitives.integer            = self.new_class_primitive_value(Some(self.primitives.any),      "Integer");
        self.primitives.method             = self.new_class_primitive_value(Some(self.primitives.any),      "Method");
        self.primitives.object             = self.new_class_primitive_value(Some(self.primitives.any),      "Object");
        self.primitives.string             = self.new_class_primitive_value(Some(self.primitives.any),      "String");

        self.primitives.array    = self.new_generic_primitive_value("Array",  Box::new([Box::from("__type__")]), &array::create);
        self.primitives.nullable = self.new_generic_primitive_value("Option", Box::new([Box::from("__type__")]), &nullable::create);

        self.primitives.array_any = {
            let mut array = self.primitives.array;
            array.data_generic_primitive_mut().call(self, Box::new([self.primitives.any])).ok().unwrap().read().ok().unwrap()
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

        let Primitives { any, class, integer, string, .. } = self.primitives;
        builder::function(self, "assert",  [any],     &assert);
        builder::function(self, "error",   [any],     &error);
        builder::function(self, "eval",    [string],  &eval);
        builder::function(self, "exec",    [string],  &exec);
        builder::function(self, "exit",    [integer], &exit);
        builder::function(self, "include", [string],  &include);
        builder::function(self, "new",     [class],   &new);
        builder::function(self, "print",   [any],     &print);
    }
}

fn assert<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    if !arguments[0].data_boolean() {
        panic!();
    }

    Ok(engine.undefined())
}

fn error<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.error, "{}", message).unwrap();
    Ok(engine.undefined())
}

fn eval<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.parser, 1, &build::expression, &arguments[0].data_string());
    Ok(match engine.run(code) {
        Some(reference) => reference,
        None => engine.undefined(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.parser, 0, &build::program, &arguments[0].data_string());
    engine.run(code);
    Ok(engine.undefined())
}

fn exit<'a>(_: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    process::exit(*arguments[0].data_integer() as i32);
}

fn include<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let code = Code::from_file(&engine.parser, 0, &build::program, &arguments[0].data_string()).unwrap();
    engine.run(code);
    Ok(engine.undefined())
}

fn new<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_object(arguments[0]))
}

fn print<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.output, "{}", message).unwrap();
    Ok(engine.undefined())
}
