mod any;
mod array;
mod boolean;
mod class;
mod function;
mod generic;
mod integer;
mod method;
mod object;
mod string;

use crate::code::Code;
use crate::nodes::build;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

use std::process;

pub struct Primitives<'a> {
    pub any:      GcValue<'a>,
    pub array:    GcValue<'a>,
    pub boolean:  GcValue<'a>,
    pub class:    GcValue<'a>,
    pub function: GcValue<'a>,
    pub generic:  GcValue<'a>,
    pub method:   GcValue<'a>,
    pub object:   GcValue<'a>,
    pub integer:  GcValue<'a>,
    pub string:   GcValue<'a>,
}

impl<'a> Primitives<'a> {
    pub fn new() -> Self {
        Self {
            any:      GcValue::null(),
            array:    GcValue::null(),
            boolean:  GcValue::null(),
            class:    GcValue::null(),
            function: GcValue::null(),
            generic:  GcValue::null(),
            method:   GcValue::null(),
            object:   GcValue::null(),
            integer:  GcValue::null(),
            string:   GcValue::null(),
        }
    }
}

impl GcTrace for Primitives<'_> {
    fn trace(&mut self) {
        for class in [self.any, self.array, self.boolean, self.class, self.function, self.generic, self.integer, self.method, self.object, self.string].iter_mut() {
            class.trace();
        }
    }
}

impl<'a> Engine<'a> {
    pub fn add_constant_value(&mut self, name: &str, value: GcValue<'a>) {
        let reference = self.new_constant(value);
        self.add_variable(name, reference);
    }

    fn add_constant_primitive<const N: usize>(&mut self, name: &str, parameters: [GcValue<'a>; N], callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) {
        let primitive = self.new_primitive(name, Box::new(parameters), callback);
        self.add_variable(name, primitive);
    }

    fn add_method_primitive<const N: usize>(&mut self, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) {
        let primitive = self.new_primitive(&name, Box::new(parameters), callback).get_value();
        value.data_class_mut().methods.insert(name.to_string(), primitive);
    }

    pub fn populate(&mut self) {
        self.primitives.class = self.new_class_primitive_value("Class");
        self.primitives.any   = self.new_class_primitive_value("Any");

        self.primitives.array    = self.new_class_primitive_value("Array");
        self.primitives.boolean  = self.new_class_primitive_value("Boolean");
        self.primitives.function = self.new_class_primitive_value("Function");
        self.primitives.generic  = self.new_class_primitive_value("Generic");
        self.primitives.integer  = self.new_class_primitive_value("Integer");
        self.primitives.method   = self.new_class_primitive_value("Method");
        self.primitives.object   = self.new_class_primitive_value("Object");
        self.primitives.string   = self.new_class_primitive_value("String");

        self.primitives.class.class = self.primitives.class;
        self.primitives.class.data_class_mut().parent = Some(self.primitives.any);
        self.primitives.any.data_class_mut().parent = None;

        any::populate(self);
        array::populate(self);
        boolean::populate(self);
        class::populate(self);
        function::populate(self);
        integer::populate(self);
        object::populate(self);
        string::populate(self);

        let Primitives { any, class, integer, string, .. } = self.primitives;
        self.add_constant_primitive("assert",  [any],     &assert);
        self.add_constant_primitive("error",   [any],     &error);
        self.add_constant_primitive("eval",    [string],  &eval);
        self.add_constant_primitive("exec",    [string],  &exec);
        self.add_constant_primitive("exit",    [integer], &exit);
        self.add_constant_primitive("include", [string],  &include);
        self.add_constant_primitive("new",     [class],   &new);
        self.add_constant_primitive("print",   [any],     &print);
    }
}

fn assert<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    if !arguments[0].data_boolean() {
        panic!();
    }

    Ok(engine.undefined())
}

fn error<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.error, "{}", message).unwrap();
    Ok(engine.undefined())
}

fn eval<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.parser, 1, &build::expression, &arguments[0].data_string());
    Ok(match engine.run(code) {
        Some(reference) => reference,
        None => engine.undefined(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.parser, 0, &build::program, &arguments[0].data_string());
    engine.run(code);
    Ok(engine.undefined())
}

fn exit<'a>(_: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    process::exit(*arguments[0].data_integer() as i32);
}

fn include<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let code = Code::from_file(&engine.parser, 0, &build::program, &arguments[0].data_string()).unwrap();
    engine.run(code);
    Ok(engine.undefined())
}

fn new<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_object(arguments[0]))
}

fn print<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.output, "{}", message).unwrap();
    Ok(engine.undefined())
}
