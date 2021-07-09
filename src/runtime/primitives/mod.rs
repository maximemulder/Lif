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
use crate::runtime::data::GenericPrimitive;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;
use crate::walker::build;

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
    pub generic:            GcValue<'a>,
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
            generic:            GcValue::null(),
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
        self.primitives.class = self.new_class_value(Some("Class"), None);
        self.primitives.any   = self.new_class_value(Some("Any"), None);

        self.primitives.class.class = self.primitives.class;
        self.primitives.class.data_class_mut().set_parent(self.primitives.any);

        self.primitives.boolean  = self.new_class_value(Some("Boolean"),  Some(self.primitives.any));
        self.primitives.file     = self.new_class_value(Some("File"),     Some(self.primitives.any));
        self.primitives.float    = self.new_class_value(Some("Float"),    Some(self.primitives.any));
        self.primitives.function = self.new_class_value(Some("Function"), Some(self.primitives.any));
        self.primitives.generic  = self.new_class_value(Some("Generic"),  Some(self.primitives.any));
        self.primitives.integer  = self.new_class_value(Some("Integer"),  Some(self.primitives.any));
        self.primitives.method   = self.new_class_value(Some("Method"),   Some(self.primitives.any));
        self.primitives.object   = self.new_class_value(Some("Object"),   Some(self.primitives.any));
        self.primitives.string   = self.new_class_value(Some("String"),   Some(self.primitives.any));

        self.primitives.array    = self.new_generic_value(Some("Array"),  Box::new([Box::from("T")]), GenericPrimitive::new(&array::create));
        self.primitives.nullable = self.new_generic_value(Some("Option"), Box::new([Box::from("T")]), GenericPrimitive::new(&nullable::create));

        self.primitives.array_any = {
            let array = self.primitives.array;
            array.clone().data_generic_mut().call(self, array, &mut [self.primitives.any]).ok().unwrap().get_value()
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

fn assert<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    if !arguments[0].data_boolean() {
        panic!();
    }

    Ok(engine.undefined())
}

fn error<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.error, "{}", message).unwrap();
    Ok(engine.undefined())
}

fn eval<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.grammar, engine.grammar.expression, &build::expression, &arguments[0].data_string());
    Ok(match engine.run(code) {
        Some(reference) => reference,
        None => engine.undefined(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.grammar, engine.grammar.program, &build::program, &arguments[0].data_string());
    engine.run(code);
    Ok(engine.undefined())
}

fn exit<'a>(_: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    process::exit(*arguments[0].data_integer() as i32);
}

fn include<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    engine.run_frame(engine.scope().parent().unwrap(), |engine| {
        let code = Code::from_file(&engine.grammar, engine.grammar.program, &build::program, &arguments[0].data_string()).unwrap();
        engine.run(code);
    });

    Ok(engine.undefined())
}

fn new<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_object(arguments[0]))
}

fn print<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.output, "{}", message).unwrap();
    Ok(engine.undefined())
}
